#include "isogram.h"
#include <ctype.h>
#include <stdio.h>
#include <string.h>

static bool is_repeating(char, int, const char[]);

static bool is_repeating(char c, int start, const char phrase[]) {
  for (int i = start; phrase[i] != '\0'; ++i) {
    if (!isalpha(c))
      return false;
    if (c == tolower(phrase[i]))
      return true;
  }

  return false;
}

bool is_isogram(const char phrase[]) {
  if (phrase == NULL)
    return false;

  for (int i = 0; phrase[i] != '\0'; ++i) {
    if (is_repeating(tolower(phrase[i]), i + 1, phrase))
      return false;
  }

  return true;
}
