from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="ca_annealig",
    version="1.0",
    rust_extensions=[RustExtension("ca_annealing", binding=Binding.PyO3)],
    zip_safe=False,
)