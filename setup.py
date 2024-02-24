from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="richard-fib-rs",
    version="0.1",
    rust_extensions=[RustExtension(
        ".richard_fib_rs.richard_fib_rs",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["richard_fib_rs"],
    classifiers=[
            "License :: OSI Approved :: MIT License",
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
            "Operating System :: POSIX",
            "Operating System :: MacOS :: MacOS X",
        ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)