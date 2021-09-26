import rustStore from "./wasm/rust-store/Cargo.toml";

export async function raf() {
  const { raf } = await rustStore();
  return raf();
}
