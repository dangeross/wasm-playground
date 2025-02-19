// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { WasmPlayground } from "./pkg/wasm_playground.js";

const addToDocument = function (text) {
  let pEl = document.createElement("p");
  let cEl = document.createElement("code");
  cEl.textContent = text;
  pEl.append(cEl);
  document.body.append(pEl);
}

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("./pkg/wasm_playground_bg.wasm");

  try {
    const wp = WasmPlayground.new();

    const resFr = await wp.fetchFiatRates();
    addToDocument(`fetchFiatRates: ${JSON.stringify(resFr, null, 2)}`);

    const resP = await wp.parse('lnbc10u1pn6mezksp59dla9r3s6gqkk863m36qj0we8znhjh2lp9tkpkmcqy6n6n6phqpspp5s258rrhvykeyax6n65h8tnaqrwsdcewyedr4u68e0vxgafyldw8qdqqcqzysrzjqtypret4hcklglvtfrdt85l3exc0dctdp4qttmtcy5es3lpt6uts6geg4j0zhpz29cqqqqqqqqqqqqqqyg9qxpqysgq5zzp2dr2ma4mag34wvqehk9ta6wys7fz6suvsja3smq7eysg49nx23yd56ds9m7tpyxusfupky8wjra0zy393msjw0szhncq936dv9gqf36jha');
    addToDocument(`parse: ${JSON.stringify(resP, null, 2)}`);
  } catch (e) {
    console.log(e)
  }
};
runWasm();