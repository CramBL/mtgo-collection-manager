#pragma once

#include "mtgoparser/util.hpp"

#include <boost/implicit_cast.hpp>

#include <algorithm>
#include <cassert>
#include <cstdint>
#include <iterator>
#include <optional>
#include <span>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#ifdef __llvm__
#define LLVM_ASSUME(expr) __builtin_assume(expr)
#else
#define LLVM_ASSUME(expr) ((void)0)
#endif

namespace mtgo::csv {


// Splits a string into a vector of sub-strings based on a delimiter
[[nodiscard]] inline auto into_substr_vec(const std::string &str, char delimiter) -> std::vector<std::string>
{
  std::vector<std::string> sub_strs;
  std::size_t start = 0;
  std::size_t end = str.find(delimiter);

  while (end != std::string::npos) {
    sub_strs.emplace_back(str.substr(start, end - start));
    start = end + 1;
    end = str.find(delimiter, start);
  }

  // Add the last token
  sub_strs.emplace_back(str.substr(start));

  return sub_strs;
}

using opt_float_t = std::optional<float>;
using opt_uint_t = std::optional<uint16_t>;

using tup_quant_and_prices_t = std::tuple<opt_uint_t, opt_float_t, opt_float_t>;

/**
 * @brief Parse a string of the form "[quantity]goatbots_price;scryfall_price" into a tuple of the form: {quantity,
 * goatbots_price, scryfall_price}.
 *
 * @param str
 * @return tup_quant_and_prices_t
 */
[[nodiscard]] inline auto parse_quant_and_prices(const std::string &str) -> tup_quant_and_prices_t
{

  opt_uint_t quantity;
  std::size_t start = 0;

  if (str[0] == '[') [[unlikely]] {
    start = str.find(']');
    auto quant = util::sv_to_uint<uint16_t>(str.substr(1, start));
    {
      [[maybe_unused]] bool quant_has_value = quant.has_value();
      assert(quant_has_value);
      LLVM_ASSUME(quant_has_value);
    }
    quantity.emplace(quant.value());
    ++start;
  }

  constexpr char delimiter = ';';
  const std::size_t delim_pos = str.find(delimiter);
  const std::string gb_price_str = str.substr(start, delim_pos);

  {// Removes the out of bounds checks and exception instructions from the assembly (https://godbolt.org/z/GP5jfPz57)
    [[maybe_unused]] const std::size_t str_size = str.size();
    LLVM_ASSUME(delim_pos < str_size);
  }
  const std::string scryfall_opt_str = str.substr(delim_pos + 1);


  opt_float_t gb_price =
    gb_price_str == "-" ? boost::implicit_cast<opt_float_t>(std::nullopt) : std::stof(gb_price_str);
  opt_float_t scryfall_price =
    scryfall_opt_str == "-" ? boost::implicit_cast<opt_float_t>(std::nullopt) : std::stof(scryfall_opt_str);

  return { quantity, gb_price, scryfall_price };
}

/**
 * @brief Parse a span of strings of the form "[quantity]goatbots_price;scryfall_price" into a vector of tuples
 * of the form: {quantity, goatbots_price, scryfall_price}.
 *
 * @param span_of_str
 * @return std::vector<tup_quant_and_prices_t> A vector of tuples of the form: {quantity, goatbots_price,
 * scryfall_price}
 */
[[nodiscard]] inline auto quant_and_prices_from_span(const std::span<std::string> &span_of_str)
  -> std::vector<tup_quant_and_prices_t>
{
  std::vector<tup_quant_and_prices_t> quant_and_prices;
  quant_and_prices.reserve(span_of_str.size());

  std::transform(
    span_of_str.begin(), span_of_str.end(), std::back_inserter(quant_and_prices), [](const std::string &str) {
      return parse_quant_and_prices(str);
    });

  return quant_and_prices;
}


}// namespace mtgo::csv