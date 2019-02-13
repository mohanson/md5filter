import subprocess
import sys


def call(command):
    print(command)
    r = subprocess.call(command, shell=True)
    if r != 0:
        sys.exit(r)


def make():
    call('cargo fmt')
    call('cargo clippy')
    call('cargo build')


def main():
    make()


if __name__ == '__main__':
    main()
