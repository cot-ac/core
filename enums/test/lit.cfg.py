import os
import lit.formats

config.name = "enums"
config.test_format = lit.formats.ShTest(True)
config.suffixes = ['.mlir']
config.test_source_root = os.path.dirname(__file__)

cot_build_dir = os.environ.get('COT_BUILD_DIR',
    os.path.join(os.path.dirname(__file__), '..', '..', '..', 'cot-dev', 'build'))
config.substitutions.append(
    ('%cir-opt', os.path.join(cot_build_dir, 'tools', 'cir-opt', 'cir-opt')))
config.substitutions.append(
    ('%FileCheck', '/opt/homebrew/opt/llvm@20/bin/FileCheck'))
