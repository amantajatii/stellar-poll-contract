import {
  BASE_FEE,
  Contract,
  Networks,
  TransactionBuilder,
  nativeToScVal,
  rpc as StellarRpc,
  scValToNative,
} from "@stellar/stellar-sdk";
import {
  getAddress,
  getNetwork,
  isConnected,
  requestAccess,
  signTransaction,
} from "@stellar/freighter-api";
import {
  BarChart3,
  Check,
  CircleOff,
  RefreshCcw,
  RotateCcw,
  Send,
  Wallet,
} from "lucide";
import "./styles.css";

const CONTRACT_ID = "CD4ZRYTEHTLKPGY2ORRRYICGHMRGYVGFCHSZUWX4JQBOJUYDOGUCLEEO";
const RPC_URL = "https://soroban-testnet.stellar.org:443";
const READ_SOURCE = "GCCD76JRX7QPKH7OF6Q3CFGKD5JOA4PWHRCKPEKS3YLLFP6EVCUV6KUG";
const EXPLORER_URL = `https://lab.stellar.org/r/testnet/contract/${CONTRACT_ID}`;

const server = new StellarRpc.Server(RPC_URL);
const contract = new Contract(CONTRACT_ID);

const state = {
  wallet: null,
  network: null,
  question: "Loading poll question...",
  yesVotes: 0,
  noVotes: 0,
  totalVotes: 0,
  busy: false,
  status: "Ready",
  error: "",
  lastHash: "",
};

const app = document.querySelector("#app");

function icon(node, size = 18) {
  const children = node
    .map(([tag, attrs]) => {
      const attributes = Object.entries(attrs)
        .map(([key, value]) => `${key}="${escapeAttribute(value)}"`)
        .join(" ");
      return `<${tag} ${attributes}></${tag}>`;
    })
    .join("");

  return `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${children}</svg>`;
}

function shortAddress(address) {
  if (!address) return "Not connected";
  return `${address.slice(0, 6)}...${address.slice(-6)}`;
}

function normalizeResult(result, key) {
  if (result?.error) throw new Error(result.error.message || result.error);
  return result?.[key] ?? result;
}

function resultPercent(value) {
  if (!state.totalVotes) return 0;
  return Math.round((value / state.totalVotes) * 100);
}

function render() {
  const yesPercent = resultPercent(state.yesVotes);
  const noPercent = resultPercent(state.noVotes);

  app.innerHTML = `
    <main class="shell">
      <header class="topbar">
        <div>
          <p class="eyebrow">Soroban testnet poll</p>
          <h1>Stellar Poll Contract</h1>
        </div>
        <div class="wallet-panel">
          <span class="network-pill">${state.network || "TESTNET"}</span>
          <button class="button button-secondary" data-action="connect">
            ${icon(Wallet)}
            <span>${state.wallet ? shortAddress(state.wallet) : "Connect Freighter"}</span>
          </button>
        </div>
      </header>

      <section class="poll-layout">
        <section class="poll-main" aria-live="polite">
          <div class="question-row">
            <span class="contract-chip">${shortAddress(CONTRACT_ID)}</span>
            <a href="${EXPLORER_URL}" target="_blank" rel="noreferrer">View contract</a>
          </div>
          <h2>${escapeHtml(state.question)}</h2>
          <div class="vote-actions">
            <button class="vote-button vote-yes" data-action="vote-yes" ${state.busy ? "disabled" : ""}>
              ${icon(Check, 22)}
              <span>Vote yes</span>
            </button>
            <button class="vote-button vote-no" data-action="vote-no" ${state.busy ? "disabled" : ""}>
              ${icon(CircleOff, 22)}
              <span>Vote no</span>
            </button>
          </div>
        </section>

        <aside class="result-panel">
          <div class="panel-heading">
            <span>${icon(BarChart3)}</span>
            <h2>Live result</h2>
          </div>
          <div class="meter-group">
            <div class="meter-copy"><span>Yes</span><strong>${state.yesVotes} (${yesPercent}%)</strong></div>
            <div class="meter"><span style="width: ${yesPercent}%"></span></div>
          </div>
          <div class="meter-group">
            <div class="meter-copy"><span>No</span><strong>${state.noVotes} (${noPercent}%)</strong></div>
            <div class="meter meter-no"><span style="width: ${noPercent}%"></span></div>
          </div>
          <div class="total-row"><span>Total votes</span><strong>${state.totalVotes}</strong></div>
          <button class="button button-secondary full" data-action="refresh" ${state.busy ? "disabled" : ""}>
            ${icon(RefreshCcw)}
            <span>Refresh result</span>
          </button>
        </aside>
      </section>

      <section class="controls">
        <form class="question-form" data-action="set-question">
          <label for="question">Poll question</label>
          <div class="input-row">
            <input id="question" name="question" maxlength="160" value="${escapeAttribute(state.question === "No question set" ? "" : state.question)}" placeholder="Should Stellar be used for workshop apps?" />
            <button class="button" type="submit" ${state.busy ? "disabled" : ""}>
              ${icon(Send)}
              <span>Set question</span>
            </button>
          </div>
        </form>
        <button class="button button-danger" data-action="reset" ${state.busy ? "disabled" : ""}>
          ${icon(RotateCcw)}
          <span>Reset votes</span>
        </button>
      </section>

      <footer class="status-line ${state.error ? "is-error" : ""}">
        <span>${escapeHtml(state.error || state.status)}</span>
        ${state.lastHash ? `<a href="https://stellar.expert/explorer/testnet/tx/${state.lastHash}" target="_blank" rel="noreferrer">Last transaction</a>` : ""}
      </footer>
    </main>
  `;

  bindEvents();
}

function bindEvents() {
  document.querySelector('[data-action="connect"]').addEventListener("click", connectWallet);
  document.querySelector('[data-action="vote-yes"]').addEventListener("click", () => submitCall("vote_yes"));
  document.querySelector('[data-action="vote-no"]').addEventListener("click", () => submitCall("vote_no"));
  document.querySelector('[data-action="refresh"]').addEventListener("click", refreshResult);
  document.querySelector('[data-action="reset"]').addEventListener("click", () => submitCall("reset_votes"));
  document.querySelector('[data-action="set-question"]').addEventListener("submit", async (event) => {
    event.preventDefault();
    const question = new FormData(event.currentTarget).get("question")?.toString().trim();
    if (!question) {
      setError("Question cannot be empty.");
      return;
    }
    await submitCall("set_question", [nativeToScVal(question, { type: "string" })]);
  });
}

async function connectWallet() {
  await runTask("Connecting Freighter...", async () => {
    const connected = normalizeResult(await isConnected(), "isConnected");
    if (!connected) throw new Error("Install Freighter browser extension first.");

    const access = await requestAccess();
    state.wallet = normalizeResult(access, "address") || normalizeResult(await getAddress(), "address");
    state.network = normalizeResult(await getNetwork(), "network") || "TESTNET";

    if (state.network !== "TESTNET") {
      throw new Error(`Switch Freighter to TESTNET. Current network: ${state.network}`);
    }

    state.status = "Wallet connected.";
  });
}

async function refreshResult() {
  await runTask("Reading poll result...", async () => {
    const result = await simulateCall("get_result");
    state.question = result.question || "No question set";
    state.yesVotes = Number(result.yes_votes || 0);
    state.noVotes = Number(result.no_votes || 0);
    state.totalVotes = Number(result.total_votes || 0);
    state.status = "Result synced from Stellar testnet.";
  });
}

async function simulateCall(method, args = []) {
  const sourceAccount = await server.getAccount(READ_SOURCE);
  const tx = new TransactionBuilder(sourceAccount, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(30)
    .build();

  const simulation = await server.simulateTransaction(tx);
  if (StellarRpc.Api.isSimulationError(simulation)) {
    throw new Error(simulation.error);
  }

  return scValToNative(simulation.result.retval);
}

async function submitCall(method, args = []) {
  await runTask("Waiting for wallet signature...", async () => {
    if (!state.wallet) await connectWallet();

    const sourceAccount = await server.getAccount(state.wallet);
    const tx = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: Networks.TESTNET,
    })
      .addOperation(contract.call(method, ...args))
      .setTimeout(60)
      .build();

    const preparedTx = await server.prepareTransaction(tx);
    const signed = await signTransaction(preparedTx.toXDR(), {
      network: "TESTNET",
      networkPassphrase: Networks.TESTNET,
      accountToSign: state.wallet,
    });

    const signedXdr = signed?.signedTxXdr || signed?.signedXDR || signed;
    const signedTx = TransactionBuilder.fromXDR(signedXdr, Networks.TESTNET);
    const response = await server.sendTransaction(signedTx);

    if (response.status !== "PENDING") {
      throw new Error(response.errorResult || `Transaction status: ${response.status}`);
    }

    const finalStatus = await server.pollTransaction(response.hash, {
      attempts: 15,
      sleepStrategy: () => 800,
    });

    if (finalStatus.status !== StellarRpc.Api.GetTransactionStatus.SUCCESS) {
      throw new Error(`Transaction failed: ${finalStatus.status}`);
    }

    state.lastHash = response.hash;
    state.status = "Transaction confirmed.";
    await refreshResult();
  });
}

async function runTask(message, task) {
  state.busy = true;
  state.status = message;
  state.error = "";
  render();

  try {
    await task();
  } catch (error) {
    setError(error.message || String(error));
  } finally {
    state.busy = false;
    render();
  }
}

function setError(message) {
  state.error = message;
  state.status = "Needs attention";
  render();
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"]/g, (char) => ({
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
  })[char]);
}

function escapeAttribute(value) {
  return escapeHtml(value).replace(/'/g, "&#39;");
}

render();
refreshResult();
