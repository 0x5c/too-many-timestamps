#compdef timestamps

local input_types='(-S --seconds -M --milliseconds -U --microseconds -N --nanoseconds -D --discord -T --twitter)'

_arguments -C \
    '(-h --help)'{-h,--help}'[Print help information]' \
    '(-V --version)'{-V,--version}'[Print version information]' \
    "$input_types"{-S,--seconds}'[Interpret input as seconds (the default)]' \
    "$input_types"{-M,--milliseconds}'[Interpret input as milliseconds]' \
    "$input_types"{-U,--microseconds}'[Interpret input as microseconds]' \
    "$input_types"{-N,--nanoseconds}'[Interpret input as nanoseconds]' \
    "$input_types"{-D,--discord}'[Interpret input as a Discord ID]' \
    "$input_types"{-T,--twitter}'[Interpret input as a Twitter Snowflake]' \
    && ret=0

return ret
