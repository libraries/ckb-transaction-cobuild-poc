import { common } from '@ckb-lumos/common-scripts';
import { transferSpore } from '@spore-sdk/core';
import { setupInputCell } from './tmBuild';
import { config, configTypedMessageLockDemo } from './tmConfig';
import { tmAccounts } from './tmWallet';
const { registerCustomLockScriptInfos } = common;

async function main() {
    registerCustomLockScriptInfos([
        {
            codeHash: configTypedMessageLockDemo.script.codeHash,
            hashType: configTypedMessageLockDemo.script.hashType,
            lockScriptInfo: {
                CellCollector: null,
                setupInputCell: setupInputCell,
                prepareSigningEntries: null,
                setupOutputCell: null,
            },
        },
    ])

    let { txSkeleton } = await transferSpore({
        outPoint: {
            txHash: '0x16bd44f98150c03249f679877e147c4aee2b97557eaab3c16ce5906a5929b4be',
            index: '0x0',
        },
        toLock: tmAccounts.bob.lock,
        config: config,
    });
    let hash = await tmAccounts.alice.signAndSendTransaction(txSkeleton);
    console.log(`Spore transfered at: https://pudge.explorer.nervos.org/transaction/${hash}`);
}

main()
