# Reading the contents of the uploaded file
with open("output.txt", "r") as file:
    lines = file.readlines()

# Extracting all lines that match the pattern
matching_lines = [line.strip() for line in lines if "Setup Phase took" in line and "amortized time" in line]

import re

# Extracting the amortized time values for "Setup Phase"
setup_times = [float(re.search(r"Setup Phase took \d+ ms, amortized time (\d+\.\d+) ms", line).group(1))
               for line in lines if "Setup Phase took" in line]

# Extracting the amortized time values for "Online Phase"
online_phase_times = [float(re.search(r"Online Phase took \d+ ms, amortized time (\d+\.\d+) ms", line).group(1))
                      for line in lines if "Online Phase took" in line]
# Extracting the "End to end amortized time" values
end_to_end_times = [float(re.search(r"End to end amortized time (\d+\.\d+) msEnd to end", line).group(1))
                    for line in lines if "End to end amortized time" in line]


# Provided order of (DBEntrySize, N)
orders = [
    ("4KB", "2^14"), ("4KB", "2^16"), ("4KB", "2^18"), ("4KB", "2^20"), ("4KB", "2^22"), ("4KB", "2^24"),
    ("16KB", "2^12"), ("16KB", "2^14"), ("16KB", "2^16"), ("16KB", "2^18"), ("16KB", "2^20"), ("16KB", "2^22"),
    ("64KB", "2^10"), ("64KB", "2^12"), ("64KB", "2^14"), ("64KB", "2^16"), ("64KB", "2^18"), ("64KB", "2^20"),
    ("256KB", "2^08"), ("256KB", "2^10"), ("256KB", "2^12"), ("256KB", "2^14"), ("256KB", "2^16"), ("256KB", "2^18")
]


paired_values_setup = list(zip(orders, setup_times))
paired_values_online = list(zip(orders, online_phase_times))
paired_values_end_to_end = list(zip(orders, end_to_end_times))


# Saving the results to a file
with open("output_extracted.txt", "w") as file:
    for (db_size, n), setup_time, online_time, end_to_end_time in zip(orders, setup_times, online_phase_times, end_to_end_times):
        file.write(f"DBEntrySize: {db_size}, N: {n} -> "
                   f"Setup Phase Amortized Time: {setup_time} ms, "
                   f"Online Phase Amortized Time: {online_time} ms, "
                   f"End to End Amortized Time: {end_to_end_time} ms\n")

