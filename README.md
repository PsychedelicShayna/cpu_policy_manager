# CPU Policy Manager (CPM)
A CLI utility to easily control CPU frequency policies on Linux from the terminal. After `auto-cpufreq` broke after an Arch update, I just decided to write my own solution, and not have it be a daemon but a utility to easily write fixed configurations. This project is still young. The help text should provide you with a general idea of its purpose.

State: incomplete, but functional.

A simple example to set the desired minimum and maximum frequency of your CPU.
```
sudo cpm set all freq 1.5:4.5         # GHz by default, suffixes supported, and comas ignored for convenience.
sudo cpm set all freq :3,500,000      # Only sets the max frequency, and defaults to KHz when no periods are present.
sudo cpm set all freq :3.5            # Equivalent to the above.
sudo cpm set all freq 800,000:        # Sets the minimum frequency to 0.8 GHz
```

Or to retrieve available governors and change the active one.
```
cpm get 0 gov avail
sudo cpm set all gov powersave
```

Or to view the current CPU frequency of all your cores.
```
cpm get all freq curr
```

You get the idea.

Here's the verbose description:
```
Setting Values:
cpm set <policies> <attribute> <value>

Example:
    cpm set all freq 3.0:4.5
        Sets the min and max frequency for all governors to 3.0 and 4.5

<policies> format:
    0     - A single policy number.
    0:4   - A range of policy numbers.
    0,2,5 - Specific policy number.
    all|* - Affects every policy.

<attribute> format:
    freq - CPU Frequency
    gov  - CPU Governor
    perf - CPU Performance Profile

<value> format:
    freq: <min>:<max>
        If no suffix is provided, and a period is present, defualts to GHz
        2.5:3.5 - Sets scaling min/max to 2.5 and 3.5 GHz

        If no suffix is provided, and no periods are present, defaults to KHz
        2500000:3500000 - Sets scaling min/max to 2.5/3.5 GHz

        To set only min, or only max, provide no value on the left or right.
        The inclusion of a : is mandatory however.

        :3.5 - Sets scaling max to 3.5 GHz
        2.5: - Sets scaling min to 2.5 Ghz

        Available suffixes: g, m, k, h

        :3.5g            - Sets scaling max to 3.5 GHz
        :3,500m          - Sets scaling max to 3,500 MHz        (3.5GHz)
        :3,500,000k      - Sets scaling max to 3,500,000 KHz    (3.5GHz)
        :3,500,000,000h  - Sets scaling max to 3,500,000,000 Hz (3.5GHz)

        Comas are ignored, so feel free to use them for readability.

        Suffixes can also be mixed

        2,500m:3,500,000k - Sets scaling min/max to 2.5/3.5Ghz

    gov: <governor>
        Must be a valid CPU governor. You can check available governors using
            cpm get 0 gov avail

        Then set it like this
            cpm set all gov powersave

    perf: <profile>
        Must be a valid performance profile. Get available profiles with
            cpm get 0 perf avail

        Then set it like this
            cpm set all perf balance_performance


Getting Values:
    cpm set <policies> <attribute> <value>

    <policies> format:
        0     - A single policy number.
        0:4   - A range of policy numbers.
        0,2,5 - Specific policy number.

        (all | *)   - Every policy.

    <attribute> format:
        freq - CPU Frequency
        gov  - CPU Governor
        perf - CPU Performance Profile

    <value> format:
        freq: min, max, (curr | current)
        gov:  (curr | current), (avail | available)
        perf: (curr | current), (avail | available)

        freq: min, max, (curr | current)
        gov:  (curr | current), (avail | available)
        perf: (curr | current), (avail | available)
```
