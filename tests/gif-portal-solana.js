const anchor = require('@project-serum/anchor');

// Need the system program, will talk about this soon.
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  // Create and set the provider, so that it can communicate with our frontend!
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  // Compiles our code in lib.rs and get it deployed locally
  const program = anchor.workspace.GifPortalSolana;

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString());

  // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
  await program.rpc.addGif("https://media0.giphy.com/media/l378BzHA5FwWFXVSg/200w.webp?cid=ecf05e47rn4mdjh028xsgtpant7pzgc7ar2h01biteacsuar&rid=200w.webp&ct=g", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Upvote gif at index 0
  await program.rpc.upvoteGif("0",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Upvote again gif at index 0
  await program.rpc.upvoteGif("0",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList)
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