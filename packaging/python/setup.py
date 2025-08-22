from setuptools import setup, find_packages
import os
import sys
from pathlib import Path

# Read the README for long description
readme_path = Path(__file__).parent.parent.parent / "README.md"
long_description = ""
if readme_path.exists():
    with open(readme_path, "r", encoding="utf-8") as f:
        long_description = f.read()

# Platform-specific library name
def get_library_name():
    """Get the platform-specific library name"""
    system = sys.platform
    if system == "darwin":
        return "libmermaid_it.dylib"
    elif system == "win32":
        return "mermaid_it.dll"
    else:
        return "libmermaid_it.so"

setup(
    name="mermaid-it",
    version="0.1.0",
    author="drempuf",
    author_email="soddyque@gmail.com",
    description="Python bindings for mermaid-it - Fast Mermaid diagram rendering",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/mermaid-it",
    project_urls={
        "Bug Tracker": "https://github.com/yourusername/mermaid-it/issues",
        "Documentation": "https://github.com/yourusername/mermaid-it/blob/main/UNIFFI_BINDINGS.md",
        "Source Code": "https://github.com/yourusername/mermaid-it",
    },
    packages=find_packages(where="../../bindings/python"),
    package_dir={"": "../../bindings/python"},
    package_data={
        "mermaid_it": [get_library_name(), "*.pyi"],
    },
    python_requires=">=3.7",
    install_requires=[],
    extras_require={
        "dev": [
            "pytest>=7.0",
            "black>=22.0",
            "mypy>=0.990",
            "pylint>=2.15",
        ],
    },
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Multimedia :: Graphics :: Graphics Conversion",
    ],
    keywords="mermaid diagram visualization svg png rendering",
)