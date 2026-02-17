import * as anchor from "@coral-xyz/anchor";
export const getFundsToWallet = async (connection: anchor.web3.Connection, wallet: anchor.web3.PublicKey, amount: number) => {

    const dropSignature = await connection.requestAirdrop(wallet, anchor.web3.LAMPORTS_PER_SOL * amount);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
        {
            signature: dropSignature,
            blockhash: latestBlockhash.blockhash,
            lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
        }, "confirmed");
};