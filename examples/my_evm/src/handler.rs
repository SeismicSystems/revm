use revm::{
    context::{
        result::{EVMError, HaltReason, InvalidTransaction},
        JournalOutput,
    },
    context_interface::{ContextTr, JournalTr},
    handler::{
        instructions::InstructionProvider, EthFrame, EvmTr, FrameResult, Handler,
        PrecompileProvider,
    },
    inspector::{Inspector, InspectorEvmTr, InspectorHandler},
    interpreter::{interpreter::EthInterpreter, InterpreterResult},
    Database,
};

pub struct MyHandler<EVM> {
    pub _phantom: core::marker::PhantomData<EVM>,
}

impl<EVM> Default for MyHandler<EVM> {
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<EVM> Handler for MyHandler<EVM>
where
    EVM: EvmTr<
        Context: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>>,
        Precompiles: PrecompileProvider<EVM::Context, Output = InterpreterResult>,
        Instructions: InstructionProvider<
            Context = EVM::Context,
            InterpreterTypes = EthInterpreter,
        >,
    >,
{
    type Evm = EVM;
    type Error = EVMError<<<EVM::Context as ContextTr>::Db as Database>::Error, InvalidTransaction>;
    type Frame = EthFrame<
        EVM,
        EVMError<<<EVM::Context as ContextTr>::Db as Database>::Error, InvalidTransaction>,
        <EVM::Instructions as InstructionProvider>::InterpreterTypes,
    >;
    type HaltReason = HaltReason;

    fn reward_beneficiary(
        &self,
        _evm: &mut Self::Evm,
        _exec_result: &mut FrameResult,
    ) -> Result<(), Self::Error> {
        // Skip beneficiary reward
        Ok(())
    }
}

impl<EVM> InspectorHandler for MyHandler<EVM>
where
    EVM: InspectorEvmTr<
        Inspector: Inspector<<<Self as Handler>::Evm as EvmTr>::Context, EthInterpreter>,
        Context: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>>,
        Precompiles: PrecompileProvider<EVM::Context, Output = InterpreterResult>,
        Instructions: InstructionProvider<
            Context = EVM::Context,
            InterpreterTypes = EthInterpreter,
        >,
    >,
{
    type IT = EthInterpreter;
}

#[cfg(test)]
mod tests {
    use revm::bytecode::opcode;
    use revm::context::{BlockEnv, CfgEnv, TxEnv};
    use revm::context::{result::EVMError, Context, Evm};
    use revm::database::{BenchmarkDB, EmptyDB, BENCH_CALLER, BENCH_TARGET};
    use revm::handler::MainnetHandler;
    use revm::interpreter::{CallOutcome, Gas, InstructionResult, InterpreterResult};
    use revm::primitives::{Address, TxKind, U256};
    use revm::primitives::{hardfork::SpecId, Bytes};

    use revm::state::Bytecode;
    use revm::{ExecuteEvm, Journal, MainBuilder, MainContext};

    use crate::MyEvm;

    use super::*;

    /// Creates frame result.
    fn call_last_frame_return(
        ctx: Context<BlockEnv, TxEnv, CfgEnv, EmptyDB, Journal<EmptyDB>, ()>,
        instruction_result: InstructionResult,
        gas: Gas,
    ) -> Gas {
        let mut evm = MyEvm::new(ctx, ()); 

        let mut exec_result = FrameResult::Call(CallOutcome::new(
            InterpreterResult {
                result: instruction_result,
                output: Bytes::new(),
                gas,
            },
            0..0,
        ));

        let handler = MainnetHandler::<_, EVMError<_, InvalidTransaction>, EthFrame<_, _, _>>::default();

        handler
            .last_frame_result(&mut evm, &mut exec_result)
            .unwrap();
        handler.refund(&mut evm, &mut exec_result, 0);
        *exec_result.gas()
    }

    #[test]
    fn test_revert_gas() {
        let ctx = Context::mainnet()
            .modify_tx_chained(|tx| {
                tx.gas_limit = 199_000;
                tx.gas_price = 20;
                tx.caller = BENCH_CALLER;
                tx.kind = TxKind::Call(BENCH_TARGET);
                tx.gas_priority_fee = None;
            })
            .with_db(BenchmarkDB::new_bytecode(Bytecode::new_legacy(
                [opcode::POP].into(),
            )));

        let mut evm = ctx.build_mainnet();
        
        let output = evm.replay().unwrap();

        println!("output: {output:#?}");


        //let gas = call_last_frame_return(ctx, InstructionResult::Revert, Gas::new(90));
        //assert_eq!(gas.remaining(), 90);
        //assert_eq!(gas.spent(), 10);
        //assert_eq!(gas.refunded(), 0);
    }
}
