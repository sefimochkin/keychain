#!/usr/bin/env python3
import pathlib

from enum import Enum

def find_dependencies(lines, dependencies_list):
    indice = []
    for i, line in enumerate(lines):
        for dependency in dependencies_list:
            if dependency in line:
                indice.append(i)

    return indice

def find_tests_block(lines):
    starting_index = -1
    for (i, line) in enumerate(lines):
        if "#[cfg(test)]" in line:
            starting_index = i
        
    finishing_index = -1
    parenthesis_count = 0
    for (i, line) in enumerate(lines[starting_index + 1: len(lines)]):
        if '{' in line:
            parenthesis_count += line.count('{')
        if '}' in line:
            parenthesis_count -= line.count('}')

        if parenthesis_count == 0:
            finishing_index = starting_index + 2 + i
    
    return (starting_index, finishing_index)

def get_base_dir():
    return str(pathlib.Path(__file__).parent.absolute()) + '/'

def get_keychain_dir():
    return str(pathlib.Path(__file__).parent.parent.parent.absolute()) + '/'

class Section(Enum):
    PATH = 1
    TEST_FILES = 2
    FILES_WITH_TESTS = 3
    TOML_FILES = 4
    DEPENDENCIES = 5
    EMPTY = 6

def get_section(line):
    if '[path]' in line:
        return Section.PATH
    if '[test_files]' in line:
        return Section.TEST_FILES
    if '[files_with_tests]' in line:
        return Section.FILES_WITH_TESTS
    if '[toml_files]' in line:
        return Section.TOML_FILES
    if '[dependencies]' in line:
        return Section.DEPENDENCIES
    if line.strip(' ') == '\n':
        return Section.EMPTY

    return None

def get_settings():
    path = ""
    files = {
        Section.TEST_FILES: [], 
        Section.FILES_WITH_TESTS: [], 
        Section.TOML_FILES: [], 
        Section.DEPENDENCIES: []
    }

    current_path = get_base_dir()

    with open(current_path + 'settings.cfg', 'r') as f:
        lines = f.readlines()

        current_section = None
        for line in lines:
            section = get_section(line)

            if section:
                current_section = section
                continue

            value = line.strip('\n').strip(' ')

            if current_section == Section.PATH:
                path = value
            else:
                files[current_section].append(
                    get_keychain_dir() + path + value
                )

    return (
        files[Section.TEST_FILES], 
        files[Section.FILES_WITH_TESTS], 
        files[Section.TOML_FILES], 
        files[Section.DEPENDENCIES]
    )

