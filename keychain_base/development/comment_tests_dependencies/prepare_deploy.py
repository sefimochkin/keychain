#!/usr/bin/env python3
import os

from shared_code import *

def comment_whole_file(name):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        old = f.read()

        with open(tmp_name, 'w') as nf:
            nf.write("/*\n" + old + "\n*/\n")

    os.remove(name)
    os.rename(tmp_name, name)

def comment_tests_in_file(name):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        lines = f.readlines()

        starting_index, finishing_index = find_tests_block(lines)

        lines.insert(starting_index, '/*\n')
        lines.insert(finishing_index + 1, '*/\n')

        with open(tmp_name, 'w') as nf:
            nf.writelines(lines)

    os.remove(name)
    os.rename(tmp_name, name)

def comment_dependency_in_toml(name, dependencies_list):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        lines = f.readlines()

        indice = find_dependencies(lines, dependencies_list)
        for i in indice:
            lines[i] = '# ' + lines[i]

        with open(tmp_name, 'w') as nf:
            nf.writelines(lines)

    os.remove(name)
    os.rename(tmp_name, name)


def main():
    test_files, files_with_tests, toml_files, dependencies = get_settings()

    for file in test_files:
        comment_whole_file(file)

    for file in files_with_tests:
        comment_tests_in_file(file)

    for file in toml_files:
        comment_dependency_in_toml(file, dependencies)

if __name__ == "__main__":
    main()
