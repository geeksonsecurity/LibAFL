use libafl;
use libafl_qemu;
use libafl_sugar;
use pyo3::prelude::*;

const LIBAFL_CODE: &str = r#"
class BaseObserver:
    def flush(self):
        pass
    def pre_exec(self, state, input):
        pass
    def post_exec(self, state, input, exit_kind):
        pass
    def pre_exec_child(self, state, input):
        pass
    def post_exec_child(self, state, input, exit_kind):
        pass
    def name(self):
        return type(self).__name__
    def as_observer(self):
        return Observer.new_py(self)

class BaseFeedback:
    def init_state(self, state):
        pass
    def is_interesting(self, state, mgr, input, observers, exit_kind):
        return False
    def append_metadata(self, state, testcase):
        pass
    def discard_metadata(self, state, input):
        pass
    def name(self):
        return type(self).__name__
    def as_feedback(self):
        return Feedback.new_py(self)

class BaseExecutor:
    def observers(self) -> ObserversTuple:
        raise NotImplementedError('Implement this yourself')
    def run_target(self, fuzzer, state, mgr, input) -> ExitKind:
        raise NotImplementedError('Implement this yourself')
    def as_executor(self):
        return Executor.new_py(self)

class BaseStage:
    def perform(self, fuzzer, executor, state, manager, corpus_idx):
        pass
    def as_stage(self):
        return Stage.new_py(self)

class BaseMutator:
    def mutate(self, state, input, stage_idx):
        pass
    def post_exec(self, state, stage_idx, corpus_idx):
        pass
    def as_mutator(self):
        return Mutator.new_py(self)

class FnStage(BaseStage):
    def __init__(self, fn):
        self.fn = fn
    def __call__(self, fuzzer, executor, state, manager, corpus_idx):
        self.fn(fuzzer, executor, state, manager, corpus_idx)
    def perform(self, fuzzer, executor, state, manager, corpus_idx):
        self.fn(fuzzer, executor, state, manager, corpus_idx)
"#;

#[pymodule]
#[pyo3(name = "pylibafl")]
pub fn python_module(py: Python, m: &PyModule) -> PyResult<()> {
    let modules = py.import("sys")?.getattr("modules")?;

    let sugar_module = PyModule::new(py, "sugar")?;
    libafl_sugar::python_module(py, sugar_module)?;
    m.add_submodule(sugar_module)?;

    modules.set_item("pylibafl.sugar", sugar_module)?;

    let qemu_module = PyModule::new(py, "qemu")?;
    libafl_qemu::python_module(py, qemu_module)?;
    m.add_submodule(qemu_module)?;
    
    modules.set_item("pylibafl.qemu", qemu_module)?;

    let libafl_module = PyModule::from_code(py, LIBAFL_CODE, "libafl", "libafl")?;
    libafl::pybind::python_module(py, libafl_module)?;
    m.add_submodule(libafl_module)?;
    
    modules.set_item("pylibafl.libafl", libafl_module)?;
    
    Ok(())
}
