# Day 7: No Space Left On Device (https://adventofcode.com/2022/day/7)
# Conditions:
# - The input represents commandos and outputs in a windows scheme, which are seperated by line
# - Available commandos are "cd" and "ls" and start with "$"
# - Outputs list directories and files; Output of a directory starts with "dir" and then name of the directory;
#   Output of a file starts with the size of a file and then the name
# - The total size of a directory is the sum of the sizes of the files it contains directly or indirectly


def main():
    print("Advent of Code - Day 7")
    file_task_1 = open('input.txt', 'r')
    file_task_2 = open('input.txt', 'r')
    task_1(file_task_1)
    task_2(file_task_2)

def get_file_system_directories(file):
    # Create a list of directories by judging the inputs
    current_path = ""
    directories = {"/home": 0}
    for line in file.read().splitlines():
        print(current_path)
        line = line.split()
        if line[0] == "$":
            if line[1] == "ls":
                pass
            else:
                if line[2] == "..":
                    # Find index of last occurrence of "/" and create new string up until that index
                    current_path = current_path[:current_path.rindex("/")]
                elif line[2] == "/":
                    current_path = "/home"
                else:
                    current_path = current_path + "/" + line[2]
                    directories[current_path] = 0
        else:
            if line[0] != "dir":
                temp_path = current_path
                # Update all parent directories
                while temp_path != "":
                    directories[temp_path] += int(line[0])
                    temp_path = temp_path[:temp_path.rindex("/")]
    return directories


# Task 1: Get sum of all directories with a total size of at most 100000
def task_1(file):
    directories = get_file_system_directories(file)
    sum_small_directories = 0
    for _, directory in directories.items():
        if directory < 100000:
            sum_small_directories += directory
    print("Task 1 result: " + str(sum_small_directories))


# Task 2: Get the smallest directory that can be deleted, in order to have at least 30000000 free space, with 70000000
#         being the total disk space available
def task_2(file):
    directories = get_file_system_directories(file)
    required_free_space = directories["/home"] - (70000000 - 30000000)  # 40000000 of the space can be used
    smallest_deletable_directory = directories["/home"]
    for _, directory in directories.items():
        if required_free_space < directory < smallest_deletable_directory:
            smallest_deletable_directory = directory
    print("Task 1 result: " + str(smallest_deletable_directory))


if __name__ == "__main__":
    main()