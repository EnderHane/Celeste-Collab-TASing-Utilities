import re
from pathlib import Path

import typer
import json

very_big_number = 99999
file_name_pattern = re.compile(r'\A([a-zA-Z]+)_(\d+)\-(\d+)')
time_stamp_pattern = re.compile(r'.*?\((\d+)\)', re.IGNORECASE)


def scan_lobby(path: str) -> dict:
    source = None
    destination = None
    result = dict()
    for p in Path(path).iterdir():
        if p.is_file() and file_name_pattern.match(p.name):
            vertice = file_name_pattern.match(p.name).groups()
            if vertice[1] in result:
                result[vertice[1]].update([(vertice[2], extract_time(p))])
            else:
                result[vertice[1]] = {vertice[2]: extract_time(p)}
            if vertice[0] == 'departure':
                source = vertice[1]
            if vertice[0] == 'terminal':
                destination = vertice[2]
    return {'source': source, 'destination': destination, 'adj_table': result}


def extract_time(path: Path) -> str:
    with path.open() as f:
        for l in f.readlines():
            if time_stamp_pattern.match(l):
                return time_stamp_pattern.match(l).group(1)
    raise RuntimeError(f'{path} does not contain a timestamp')


def main(
    path: str = typer.Argument(..., help='the folder where the lobby TAS files are'),
    destination: str = typer.Option(None, '--destination', '-d', help='the point at which you finish'),
    source: str = typer.Option(None, '--source', '-s', help='the point at which you start'),
    output: str = typer.Option('', '--output', '-o', help='the file which the extraction are put. will write NO file if not given')
    ):
    scanned = scan_lobby(path)
    source = source or scanned.get('source') or '0'
    # use the largest index as default destination 
    destination = destination or scanned.get('destination') or \
        max(map(lambda key: max(int(key), max(map(lambda key1: int(key1), scanned[key]))), scanned))
    out = {
        'lobby_path': path,
        'source': source, 
        'destination': destination,
        'adj_table': scanned.get('adj_table')
        }
    if output:
        with Path(output).open(mode='w') as wf:
            wf.write(json.dumps(out, indent=4))
            print(f'lobby data saved to {output}')
    else:
        print(json.dumps(out, indent=4))


if __name__ == '__main__':
    typer.run(main)
