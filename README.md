# RustPythonBindings
Writing a python library with a rust backend

To initialize a repo for python bindings, run:

```
conda create -n RustPythonBindings
conda activate RustPythonBindings
pip install maturin
maturin init
```

You can then install the package...


```
pip install .
```

Example:

```
import RustPythonBindings
x = RustPythonBindings.Number(10)
y = RustPythonBindings.Number(20)
z = x.add(y)
print(z.value())
```

You can also run the parallel code example...

```
python parallel.py

sequential: 17.748s
parallel: 9.220s
sequential_allow_threads: 2.445s
```