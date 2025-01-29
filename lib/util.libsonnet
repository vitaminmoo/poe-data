local isCaps(chr) = std.codepoint(chr) >= 65 && std.codepoint(chr) <= 90;
local isNum(chr) = std.codepoint(chr) >= 48 && std.codepoint(chr) <= 57;

{
  case: {
    snake(str):
      std.lstripChars(
        std.asciiLower(
          std.join(
            '', std.mapWithIndex(
              function(i, chr)
                local chrs = std.stringChars(str);
                if i == 0 || chrs[i - 1] == '_'  // first char, chars after _ = just lowercase
                then chr
                else if isCaps(chr)
                        && !isCaps(chrs[i - 1])
                then '_' + chr
                else if isCaps(chr)
                        && std.length(str) > i + 1
                        && !isCaps(chrs[i + 1])
                        && !isNum(chrs[i + 1])
                then '_' + chr
                else chr
              , std.stringChars(str)
            )
          )
        ), '_'
      ),
    ucfirst(s):
      std.asciiUpper(
        std.substr(s, 0, 1)
      ) + std.join('', std.stringChars(s)[1:]),
    pascal(x): std.join(
      '',
      std.map(
        $.case.ucfirst,
        std.split(std.asciiLower(x), '_')
      )
    ),
  },
  enumeratorToVariant(indexing, idx, enumerator):
    '    %s = %s,' % [$.case.pascal(enumerator), indexing + idx],
}
