Here is a comprehensive, deep-dive specification of the CodeCrafters "Build Your Own Shell" challenge stages. You can save this exact Markdown output to a local `OFFLINE_ROADMAP.md` file in your repository to use as your blueprint while disconnected.

---

## 📋 Detailed Stage Specifications

### Stage 1: The Prompt (`$`)

- **Goal:** Initialize the REPL interface.
- **Requirements:** Print exactly `$` (dollar sign followed by a single space) without a trailing newline. You **must** manually flush `stdout` using `io::stdout().flush().unwrap();` because Rust buffers standard output until a newline occurs.
- **Tester Expectation:** The test harness connects to your process and looks for the explicit two-character sequence `$` before issuing input.

### Stage 2: Handle Invalid Commands

- **Goal:** Prevent the shell from crashing on bad input.
- **Requirements:** If the user types any unrecognized command (e.g., `invalid_command`), your shell must output `invalid_command: command not found` to `stdout`, followed by a newline, and return to the `$` prompt.
- **Rust Warning:** Do not use `panic!()` or exit the program when parsing fails.

### Stage 3: The `exit` Built-In

- **Goal:** Provide a safe termination mechanism.
- **Requirements:** Intercept the command text. If the user types `exit 0`, cleanly terminate the shell process using `std::process::exit(0);`.
- **Edge Case:** The remote tester tests for the arguments. Your parser should look for `exit` as the command and extract the string literal `0` (or any valid integer) as the status code argument.

### Stage 4: The `echo` Built-In

- **Goal:** Basic string output parsing.
- **Requirements:** Intercept the `echo` keyword. Print all subsequent arguments separated by a single space, followed by a trailing newline.
- **Input Example:** `echo hello   world`
- **Output:** `hello world\n` (Note how your parser should automatically handle collapses of multiple consecutive spaces).

### Stage 5: The `type` Built-In (Shell Built-Ins)

- **Goal:** Identify internal shell functions.
- **Requirements:** Implement a `type` command. If the user executes `type echo`, `type exit`, or `type type`, output `xxx is a shell builtin`. If it is an unrecognized query, print `xxx: not found`.

### Stage 6: The `type` Built-In (System Binaries)

- **Goal:** Discover external applications.
- **Requirements:** Extend your `type` logic. If the input is not an internal shell built-in, fetch the system's `PATH` environment variable using `env::var("PATH")`.
- **Logic Flow:** Split the `PATH` string by the delimiter `:` (colon). Iterate through these directory paths sequentially. Check if the queried executable (like `cat` or `ls`) exists inside any of those directories and is an executable file.
- **Output Format:** If found, print `cat is /usr/bin/cat`. If missing across all directories, fall back to `cat: not found`.

### Stage 7: Run External Programs

- **Goal:** Execute code outside your shell process.
- **Requirements:** If a command is not a built-in, search the system `PATH` to locate its true location. Use Rust's `std::process::Command::new(binary_path)` to launch it.
- **Execution Contract:** Pass the extracted arguments to the process, redirect its `stdout` and `stderr` to inherit your shell's streams, and call `.wait()` to halt your loop until the child process completes execution.

### Stage 8: The `pwd` Built-In

- **Goal:** Print the present working directory.
- **Requirements:** Implement `pwd` as a shell built-in. Use `env::current_dir()` to fetch an absolute path buffer, convert it to a string cleanly, and print it out.

### Stage 9: The `cd` Built-In (Absolute & Relative)

- **Goal:** Context navigation.
- **Requirements:** Implement `cd <path>`. Use `env::set_current_dir(path)` to alter the process state.
- **Error Handling:** If the target directory does not exist, print `cd: <path>: No such file or directory`.

### Stage 10: The `cd` Built-In (Home Path Shortcut)

- **Goal:** Support home directory expansion.
- **Requirements:** Modify the `cd` handler. If the user passes `cd ~`, extract the user's home directory path using `env::var("HOME")` and route the execution to change directory to that location instead.

---

## 🚀 Advanced Tracking: Autocompletion & Redirection

If you fly through the first ten stages offline, these are the remaining features needed to complete the architecture:

### Stage 11: Single Tab Completion

- **Goal:** Complete a command name automatically.
- **Requirements:** Listen for a literal `\t` (Tab) character in raw input mode. If the partial string matches exactly one available option across your built-ins and binaries in `PATH`, inject the rest of the string into the terminal display instantly along with a space.

### Stage 12: Multiple Tab Completion

- **Goal:** Display completion alternatives.
- **Requirements:** If a double-tab occurs and multiple binaries match the prefix, print an alphabetical list of options on a newline, then re-render the prompt with the existing partial input intact. If there are zero matches, output the terminal bell character (`\a`).

### Stage 13: Redirecting `stdout` (`>`)

- **Goal:** Output routing to files.
- **Requirements:** Parse the input string for the `>` or `1>` token. Strip this indicator and the filename following it from your command arguments list. Open or create that file locally, and pass it to the child process builder using `.stdout(Stdio::from(file))` instead of inheriting terminal display output.

### Stage 14: Appending `stdout` (`>>`)

- **Goal:** Content preservation while redirecting.
- **Requirements:** Identify `>>`. Open the designated target file in "Append Mode" via Rust's `std::fs::OpenOptions::new().append(true)` so new logs concatenate cleanly to the end of the file rather than clearing past content.

### Stage 15: Redirecting `stderr` (`2>`)

- **Goal:** Separating execution faults.
- **Requirements:** Identify `2>`. Route the `.stderr()` channel of your spawned `Command` object to dump data directly to the given text file, while keeping `.stdout()` projecting to the terminal display normally.

That is the vast majority of the core structure, but CodeCrafters continuously expands their projects with **bonus extensions** to mimic real production environments.

Depending on how deeply you want to build your shell during your offline session, there are a few more advanced extensions that you can tackle. If you want to aim for a truly comprehensive, robust POSIX-like shell, save these additional specifications to your local file:

---

## 🚀 Advanced Extensions Checklist

### 1. Quotes and Escaping

Real shells allow you to pass spaces inside arguments if they are quoted.

- **Single Quotes (`'...'`):** Everything inside single quotes must be treated as a single literal string argument. For example, `echo 'hello     world'` must output `hello     world` (preserving the exact spaces). Internal backslashes have no special meaning inside single quotes.
- **Double Quotes (`"..."`):** Similar to single quotes, but backslashes (`\`) retain special escaping properties inside them. For instance, `\"` embeds a literal double quote, `\\` embeds a backslash, and `\n` embeds a newline.
- **Backslash Escaping (Outside Quotes):** An unquoted backslash escapes the very next character. For example, `echo hello\ \ \ world` must treat the spaces as part of the string argument, rendering `hello   world`.

### 2. Pipelines (`|`)

- **Goal:** Chain multiple programs together by routing data streams.
- **Requirements:** Parse the command string for the pipe `|` operator. This splits the line into two separate executable tasks (e.g., `cat files.txt | grep "error"`).
- **Rust Implementation:** You must spawn _both_ processes simultaneously using `std::process::Command`. You'll need to configure the first process to pipe its standard output (`.stdout(Stdio::piped())`), extract that pipe handle, and pass it directly into the `.stdin()` configuration of the second process before waiting for both to finish.

### 3. Parameter / Environment Variable Expansion (`$VARIABLE`)

- **Goal:** Read shell variables dynamically.
- **Requirements:** When tokenizing strings, search for unquoted `$` characters followed by an identifier name (like `$HOME` or `$PATH`). Your shell needs to look up that value using `std::env::var()` and dynamically swap the variable placeholder out for the real environment string value _before_ executing the final command.

---

### 💡 Your Offline Strategy

If you implement all 15 base stages from earlier, plus **Quoting**, **Pipelines**, and **Variable Expansion**, you will have essentially engineered a fully functioning, custom POSIX-like shell clone entirely from scratch.

Save this combined with the previous list, grab a coffee, and you're fully equipped to code entirely off the grid! Good luck with the Rust implementation—remember to watch out for string slicing boundaries when parsing those quotes!
