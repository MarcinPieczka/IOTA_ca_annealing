from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="ca_annealig",
    version="1.0",
    rust_extensions=[RustExtension("ca_annealing.ca_annealing", binding=Binding.PyO3)],
    package="ca_annealing",
    zip_safe=False,
)