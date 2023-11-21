
#include "mtgoparser/mtg.hpp"

#include <cstdint>
#include <optional>
#include <tuple>
#include <vector>

using opt_float_t = std::optional<float>;
using opt_uint_t = std::optional<uint16_t>;

using tup_quant_and_prices_t = std::tuple<opt_uint_t, opt_float_t, opt_float_t>;

namespace mtgo {


/// Helper struct to so that `CardHistory` can be constructed with designated initializers for the fields with matching
/// types.
struct [[nodiscard]] QuantityNameSet
{
  std::string quantity_;
  std::string name_;
  std::string set_;
};

/**
 * @brief Holds the history of a card in terms of its price and quantity.
 *
 */
struct [[nodiscard]] CardHistory
{
  uint32_t id_;
  std::string quantity_;
  std::string name_;
  std::string set_;
  mtg::Rarity rarity_;
  bool foil_;
  std::vector<tup_quant_and_prices_t> price_history_;

  explicit CardHistory(uint32_t id,
    QuantityNameSet &&qns,
    mtg::Rarity rarity,
    bool foil,
    std::vector<tup_quant_and_prices_t> &&price_history) noexcept
    : id_(id), quantity_(qns.quantity_), name_(std::move(qns.name_)), set_(std::move(qns.set_)), rarity_(rarity),
      foil_(foil), price_history_(std::move(price_history))
  {}

  // Move constructor
  [[nodiscard]] CardHistory(CardHistory &&other) noexcept
    : id_(other.id_), quantity_(std::move(other.quantity_)), name_(std::move(other.name_)), set_(std::move(other.set_)),
      rarity_(other.rarity_), foil_(other.foil_), price_history_(std::move(other.price_history_))
  {}

  // Delete copy && assignment constructor
  CardHistory(const CardHistory &) = delete;
  CardHistory &operator=(const CardHistory &) = delete;
};

}// namespace mtgo