#input at https://adventofcode.com/{year}/day/{num}/input
import requests

year = 2024

if __name__ == '__main__':
    s = requests.Session()

    day_num = int(input("day: "))
    session = ""
    with open(".session", "r") as file:
        session = file.read().strip()

    day = f"https://adventofcode.com/{year}/day/{day_num}/input"
    response = s.get(day, cookies={"session": session}).text

    with open(f"files/day_{day_num}", "w") as file:
        file.write(response.strip())
