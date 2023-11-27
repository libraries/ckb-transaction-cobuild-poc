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
            txHash: '0x3affc26498377ba3b25e037ff812cb4fad6f3afabf003883ebab41536bcc1be5',
            index: '0x0',
        },
        toLock: tmAccounts.bob.lock,
        config: config,
    });
    let hash = await tmAccounts.alice.signAndSendTransaction(txSkeleton);
    console.log(`Spore transfered at: https://pudge.explorer.nervos.org/transaction/${hash}`);
}

main()
