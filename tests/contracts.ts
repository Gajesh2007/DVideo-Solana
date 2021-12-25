import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Contracts } from '../target/types/contracts';

describe('contracts', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Contracts as Program<Contracts>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
