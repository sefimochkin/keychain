#!/usr/bin/env python3
import os

from shared_code import *

def uncomment_whole_file(name):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        lines = f.readlines()
        if len(lines) < 3:
            return

        first_line = lines[0]
        next_to_last = lines[-2]
        last_line = lines[-1]

        start_index = 0
        finish_index = len(lines)

        if '/*' in first_line:
            start_index = 1
        if '*/' in last_line:
            finish_index -= 1
        if '\n' == next_to_last:
            finish_index -= 1

        with open(tmp_name, 'w') as nf:
            nf.writelines(lines[start_index:finish_index])

    os.remove(name)
    os.rename(tmp_name, name)

def uncomment_tests_in_file(name):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        lines = f.readlines()

        tests_starting_index, tests_finishing_index = find_tests_block(lines)
        starting_index = tests_starting_index - 1
        finishing_index = tests_finishing_index - 2

        if '/*' in lines[starting_index]:
            del lines[starting_index]
        
        if '*/' in lines[finishing_index]:
            del lines[finishing_index]

        with open(tmp_name, 'w') as nf:
            nf.writelines(lines)

    os.remove(name)
    os.rename(tmp_name, name)

def uncomment_dependency_in_toml(name, dependencies_list):
    tmp_name = name + '.tmp'
    with open(name, 'r') as f:
        lines = f.readlines()

        indice = find_dependencies(lines, dependencies_list)
        for i in indice:
            lines[i] = lines[i][2:]

        with open(tmp_name, 'w') as nf:
            nf.writelines(lines)

    os.remove(name)
    os.rename(tmp_name, name)

def main():
    test_files, files_with_tests, toml_files, dependencies = get_settings()

    for file in test_files:
        uncomment_whole_file(file)

    for file in files_with_tests:
        uncomment_tests_in_file(file)

    for file in toml_files:
        uncomment_dependency_in_toml(file, dependencies)

if __name__ == "__main__":
    main()