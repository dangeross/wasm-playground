// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { WasmPlayground, setLogger } from "./pkg/wasm_playground.js";

const addToDocument = function (text) {
  let pEl = document.createElement("p");
  let cEl = document.createElement("code");
  cEl.textContent = text;
  pEl.append(cEl);
  document.body.append(pEl);
}

class JsEventListener {
  onEvent = (event) => {
    addToDocument(`EVENT RECEIVED: ${JSON.stringify(event)}`)
  }
}

class JsLogger {
  log = (logEntry) => {
    addToDocument(`LOG [${logEntry.level}]: ${logEntry.line}`)
  }
}

class JsSigner {
  xpub = () => { return [0] }
  deriveXpub = (derivationPath) => { return [1] }
  signEcdsa = (msg, derivationPath) => { return [2] }
  signEcdsaRecoverable = (msg) => { return [3] }
  slip77MasterBlindingKey = () => { return [4] }
  hmacSha256 = (msg, derivationPath) => { return [5] }
  eciesEncrypt = (msg) => { return [6] }
  eciesDecrypt = (msg) => { return [7] }
}

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("./pkg/wasm_playground_bg.wasm");
  const eventListener = new JsEventListener();
  const signer = new JsSigner();

  setLogger("trace");

  try {
    const wp = WasmPlayground.newWithSigner(signer);

    //const resFr = await wp.fetchFiatRates();
    //addToDocument(`fetchFiatRates: ${JSON.stringify(resFr, null, 2)}`);

    //const resFc = await wp.listFiatCurrencies();
    //addToDocument(`listFiatCurrencies: ${JSON.stringify(resFc, null, 2)}`);

    //const resF = await wp.fiat(resFc[0]);
    //addToDocument(`res: ${JSON.stringify(resF, null, 2)}`);

    const resP = await wp.parse('matt@mattcorallo.com');
    addToDocument(`parse: ${JSON.stringify(resP, null, 2)}`);

    //const resI = await wp.parseInvoice('lnbc10u1pn6mezksp59dla9r3s6gqkk863m36qj0we8znhjh2lp9tkpkmcqy6n6n6phqpspp5s258rrhvykeyax6n65h8tnaqrwsdcewyedr4u68e0vxgafyldw8qdqqcqzysrzjqtypret4hcklglvtfrdt85l3exc0dctdp4qttmtcy5es3lpt6uts6geg4j0zhpz29cqqqqqqqqqqqqqqyg9qxpqysgq5zzp2dr2ma4mag34wvqehk9ta6wys7fz6suvsja3smq7eysg49nx23yd56ds9m7tpyxusfupky8wjra0zy393msjw0szhncq936dv9gqf36jha');
    //addToDocument(`parseInvoice: ${JSON.stringify(resI, null, 2)}`);

    const resAel = await wp.addEventListener(eventListener);
    addToDocument(`addEventListener: ${JSON.stringify(resAel, null, 2)}`);

    const resFr = await wp.fetchFiatRates();
    addToDocument(`fetchFiatRates: ${JSON.stringify(resFr, null, 2)}`);

  } catch (e) {
    console.log(e)
  }
};
runWasm();