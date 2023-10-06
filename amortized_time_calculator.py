# Open and read the updated file
with open("./extracted_data_updated.txt", "r") as file:
    lines = file.readlines()

# Patterns to extract the values
end_to_end_pattern = r"End to End Amortized Time: ([\d\.]+) ms"
online_phase_pattern = r"Online Phase Amortized Time: ([\d\.]+) ms"
setup_phase_pattern = r"Setup Phase Amortized Time: ([\d\.]+) ms"

# Extract the values using regex
end_to_end_values = [float(re.search(end_to_end_pattern, line).group(1)) for line in lines if re.search(end_to_end_pattern, line)]
online_phase_values = [float(re.search(online_phase_pattern, line).group(1)) for line in lines if re.search(online_phase_pattern, line)]

# Calculate the new setup phase values
new_setup_values = [end_to_end - online for end_to_end, online in zip(end_to_end_values, online_phase_values)]

# Update the values in the file content
updated_content = []
setup_idx = 0
for line in lines:
    if "Setup Phase Amortized Time:" in line:
        updated_content.append(f"Setup Phase Amortized Time: {new_setup_values[setup_idx]} ms")
        setup_idx += 1
    else:
        updated_content.append(line.strip())

# Write the updated content back to a new file
output_filename_final = "./extracted_data_final.txt"
with open(output_filename_final, "w") as outfile:
    outfile.write("\n".join(updated_content))

