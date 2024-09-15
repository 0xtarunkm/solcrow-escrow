import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Escrow } from '../target/types/escrow';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { randomBytes } from 'crypto';
import { ASSOCIATED_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/utils/token';

describe('escrow', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  const wallet = provider.wallet as NodeWallet;

  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();

  let mint: anchor.web3.PublicKey;

  let makerAta: anchor.web3.PublicKey;

  let takerAta: anchor.web3.PublicKey;

  let seed = new anchor.BN(randomBytes(8));

  const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('escrow'),
      maker.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, 'le', 8),
    ],
    program.programId
  );

  it('Airdrop SOL to maker and taker', async () => {
    const tx = await provider.connection.requestAirdrop(
      maker.publicKey,
      1000000000
    );

    await provider.connection.confirmTransaction(tx);
    console.log(
      'Maker balance',
      await provider.connection.getBalance(maker.publicKey)
    );

    const tx2 = await provider.connection.requestAirdrop(
      taker.publicKey,
      1000000000
    );
    await provider.connection.confirmTransaction(tx2);

    console.log(
      'Taker balance',
      await provider.connection.getBalance(taker.publicKey)
    );
  });

  it('Create token and mint tokens', async () => {
    mint = await createMint(
      provider.connection,
      wallet.payer,
      provider.publicKey,
      provider.publicKey,
      6
    );
    console.log('Mint: ', mint.toBase58());

    makerAta = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet.payer,
        mint,
        maker.publicKey
      )
    ).address;

    takerAta = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet.payer,
        mint,
        taker.publicKey
      )
    ).address;

    await mintTo(
      provider.connection,
      wallet.payer,
      mint,
      makerAta,
      provider.publicKey,
      1_000_000_0
    );
  });

  it('make', async () => {
    const vault = getAssociatedTokenAddressSync(
      mint,
      escrow,
      true,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .make(seed, new anchor.BN(1_000_000))
      .accountsPartial({
        maker: maker.publicKey,
        mint,
        makerAta,
        escrow,
        vault,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    console.log('Your transaction signature ', tx);
  });

  xit('refund', async () => {
    const vault = getAssociatedTokenAddressSync(
      mint,
      escrow,
      true,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .refund()
      .accountsPartial({
        maker: maker.publicKey,
        mint,
        makerAta,
        escrow,
        vault,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    console.log('Your transaction signature ', tx);
  });

  it('take', async () => {
    const vault = getAssociatedTokenAddressSync(
      mint,
      escrow,
      true,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods
      .take()
      .accountsPartial({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mint,
        escrow,
        vault,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([taker])
      .rpc();

    console.log('Your transaction signature: ', tx);
  });
});
