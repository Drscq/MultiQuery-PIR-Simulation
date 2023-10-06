import re

# Open and read the uploaded file
with open("./output.txt", "r") as file:
    content = file.read()

# Define the patterns to search for
setup_total_time_pattern = r"Setup Phase took ([\d\.]+) ms"
local_set_num_pattern = r"Local Set Num (\d+)"
online_pattern = r"Online Phase took [\d\.]+ ms, amortized time ([\d\.]+) ms"
end_to_end_pattern = r"End to end amortized time ([\d\.]+) ms"

# Extract the required values using regex
setup_total_times = [float(time) for time in re.findall(setup_total_time_pattern, content)]
local_set_nums = [float(num) for num in re.findall(local_set_num_pattern, content)]
online_times = re.findall(online_pattern, content)
end_to_end_times = re.findall(end_to_end_pattern, content)

# Calculate the new amortized times for the setup phase
computed_setup_times = [total_time / num for total_time, num in zip(setup_total_times, local_set_nums)]

# Create a list of (DBEntrySize, N) pairs
dbentry_n_pairs = [
    "4KB, 2^14", "4KB, 2^16", "4KB, 2^18", "4KB, 2^20", "4KB, 2^22", "4KB, 2^24",
    "16KB, 2^12", "16KB, 2^14", "16KB, 2^16", "16KB, 2^18", "16KB, 2^20", "16KB, 2^22",
    "64KB, 2^10", "64KB, 2^12", "64KB, 2^14", "64KB, 2^16", "64KB, 2^18", "64KB, 2^20",
    "256KB, 2^08", "256KB, 2^10", "256KB, 2^12", "256KB, 2^14", "256KB, 2^16", "256KB, 2^18"
]

# Prepare the data to be written to the output file with the updated setup phase times
output_data_updated = []
for idx, pair in enumerate(dbentry_n_pairs):
    output_data_updated.append(f"DBEntrySize: {pair}")
    output_data_updated.append(f"Setup Phase Amortized Time: {computed_setup_times[idx]} ms")
    output_data_updated.append(f"Online Phase Amortized Time: {online_times[idx]} ms")
    output_data_updated.append(f"End to End Amortized Time: {end_to_end_times[idx]} ms")
    output_data_updated.append("----------")

# Write the updated results to a new file
output_filename_updated = "./extracted_data_updated.txt"
with open(output_filename_updated, "w") as outfile:
    outfile.write("\n".join(output_data_updated))
