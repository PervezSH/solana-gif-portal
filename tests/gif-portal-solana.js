const anchor = require('@project-serum/anchor');

const main = async() => {
  console.log("🚀 Starting test...")

  // Tell anchor to set provider
  anchor.setProvider(anchor.Provider.env());
  // Compiles our code in lib.rs and get it deployed locally
  const program = anchor.workspace.GifPortalSolana;
  // Call function, we await it which will wait for our local validator to "mine"
  const tx = await program.rpc.startStuffOff();

  console.log("📝 Your transaction signature", tx);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();