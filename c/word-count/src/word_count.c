#include "word_count.h"
#include <ctype.h>
#include <stdio.h>
#include <string.h>

/*unsigned int get_next_word(const char*, char*, unsigned int, unsigned int);*/
word_count_word_t* find_word(word_count_word_t*, const char*, int,
                             unsigned int);

word_count_word_t*
find_word(word_count_word_t* words, const char* word, int max, unsigned int len)
{
  if (words == NULL)
    return NULL;

  for (int i = 0; i < max; ++i) {
    if (!strncmp(words[i].text, word, len)) {
      return &words[i];
    }
  }

  return NULL;
}

/*
unsigned int
get_next_word(const char* src, char* dst, unsigned int dst_len,
              unsigned int start)
{
  if (src == NULL || dst == NULL)
    return 0;

  unsigned int _s = start;
  while (src[_s] != '\0' && (!isalnum(src[_s]) && src[_s] != '\''))
    ++_s;

  if (src[_s] == '\0')
    return 0;

  unsigned int _d = _s;
  while (src[_d] != '\0' && (src[_d] == '\'' || isalnum(src[_d])))
    ++_d;

  unsigned int _start = (src[_s] == '\'') ? _s + 1 : _s;
  unsigned int _end = ((src[_d - 1] == '\'') ? _d - 1 : _d) - _start;
  _end = (_end > dst_len) ? dst_len : _end;

  memset(dst, '\0', dst_len);
  for (unsigned int i = 0; i < _end; ++i)
    dst[i] = tolower(src[i + _start]);

  return _d;
}
*/

int
word_count(const char* input_text, word_count_word_t* words)
{
  unsigned int count = 0;
  memset(words, 0, sizeof(word_count_word_t) * MAX_WORDS);

  unsigned int input_len = strlen(input_text) + 1;
  char work[input_len];
  for (unsigned int i = 0; i < input_len; ++i) {
      work[i] = tolower(input_text[i]);
  }
  work[input_len] = '\0';

  const char *delim = " ,";
  for (char *word = strtok(work, delim); word != NULL; word = strtok(NULL, delim)) {
    unsigned int len = strlen(word);
    if (len > MAX_WORD_LENGTH)
      return EXCESSIVE_LENGTH_WORD;

    if (count == MAX_WORDS)
      return EXCESSIVE_NUMBER_OF_WORDS;

    word_count_word_t* wcw_p = find_word(words, word, count, len);
    if (wcw_p != NULL) {
      wcw_p->count += 1;
      continue;
    }

    memset(words[count].text, '\0', sizeof(words[count].text));
    strncpy(words[count].text, word, len);
    words[count].count = 1;
    count += 1;
  }
  return count;
}
