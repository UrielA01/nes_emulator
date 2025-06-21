import re

def strip_ppu_info(line: str) -> str:
    """
    Removes PPU and CYC parts from the end of a line.
    """
    return re.sub(r"\s+PPU:.*", "", line).rstrip()

def compare_logs(file1_path, file2_path, max_differences=5):
    with open(file1_path, "r") as f1, open(file2_path, "r") as f2:
        f1_lines = f1.readlines()
        f2_lines = f2.readlines()
    
    KNOWN_COMMANDS_LINE = 5003
    total_lines = min(len(f1_lines), len(f2_lines))
    diffs_found = 0

    for i in range(total_lines):
        # if i == KNOWN_COMMANDS_LINE:
        #     break
        your_line = f1_lines[i].rstrip()
        official_line = strip_ppu_info(f2_lines[i])

        if your_line != official_line:
            print(f"\n🟥 Difference at line {i + 1}:")
            print(f"Your log : {your_line}")
            print(f"Official : {official_line}")
            diffs_found += 1
            if diffs_found >= max_differences:
                print(f"\nStopped after {max_differences} differences.")
                break
    if len(f1_lines) != len(f2_lines) and len(f1_lines) != KNOWN_COMMANDS_LINE + 1:
        print(f"\n⚠️ Log lengths differ: your log has {len(f1_lines)}, official has {len(f2_lines)}.")
    else:
        print("\n✅ Logs match (excluding PPU/CYC info)")

if __name__ == "__main__":
    compare_logs("mynes.log", "nestest.log")
