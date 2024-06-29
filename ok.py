def remove_duplicates(input_file, output_file):
    try:
        unique_lines = {}

        # Open the input file in read mode
        with open(input_file, 'r') as f:
            for line in f:
                # Strip any leading/trailing whitespace and newline characters
                clean_line = line.strip()
                # Add the line to the dictionary with the line as key and None as value
                unique_lines[clean_line] = None

        # Open the output file in write mode
        with open(output_file, 'w') as f:
            # Write the unique lines to the output file
            for line in unique_lines:
                f.write(line + '\n')

        print(f"Duplicate removal successful. Unique lines written to {output_file}")

    except FileNotFoundError:
        print(f"Error: File '{input_file}' not found.")
    except IOError:
        print(f"Error: Unable to read or write to files.")

# Example usage:
if __name__ == "__main__":
    input_file = 'res'
    output_file = 'output.txt'
    remove_duplicates(input_file, output_file)
