const anchor = require('@project-serum/anchor');
const assert = require('assert');
const {SystemProgram} = anchor.web3;

// Mocha takes describe blocks as testing blocks and within it
// We can write tests using it blocks

describe('mycalculatordapp',() => {

  // providers is abstraction of a connection to the Solana network
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();

  // Program is an abstraction that combines the Provider, idl, and the programID (Generated when the program is built)
  const program = anchor.workspace.Mycalculatordapp;

  it('Creates a calculator', async() =>{
    await program.rpc.create("Welcome to Solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator]
    });

    // _ prefix for private class
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
    _calculator = calculator;
  });

  // For testing, we cannot directly use numbers 
  // We will cast them into Anchor big numbers
  it('Adds two numbers', async function(){
    const calculator = _calculator;

    await program.rpc.add(new anchor.BN(2),new anchor.BN(3),{
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)));
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it('Multiplies two numbers', async function(){
    const calculator = _calculator;

    await program.rpc.multiply(new anchor.BN(2),new anchor.BN(3),{
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(6)));
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it('Subtracts two numbers', async function(){
    const calculator = _calculator;

    await program.rpc.subtract(new anchor.BN(2),new anchor.BN(3),{
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(-1)));
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it('Divides two numbers', async function(){
    const calculator = _calculator;

    await program.rpc.divide(new anchor.BN(2),new anchor.BN(3),{
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(0)));
    assert.ok(account.remainder.eq(new anchor.BN(2)));
    assert.ok(account.greeting === "Welcome to Solana");
  });
})