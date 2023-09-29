#pragma once

#include "mtgoparser/goatbots.hpp"
#include "mtgoparser/mtgo/card.hpp"
#include "mtgoparser/mtgo/xml.hpp"

#include <glaze/glaze.hpp>
#include <spdlog/spdlog.h>

#include <algorithm>
#include <numeric>
#include <string>
#include <utility>
#include <vector>

namespace mtgo {
class Collection
{
  // Member variables
  // TODO: Add timestamp
  std::vector<Card> cards_;

  // Memoization
  // Don't have much more than 4 billion cards users please
  std::optional<uint32_t> total_quantity_ = std::nullopt;
  // Don't have much more than 60k of the same card users please
  std::optional<std::vector<uint16_t>> card_quantity_ = std::nullopt;

public:
  [[nodiscard]] explicit Collection(std::vector<Card> &&cards) noexcept : cards_{ std::move(cards) } {}
  [[nodiscard]] explicit Collection(const std::string &json_str) noexcept
    : cards_{ glz::read_json<std::vector<Card>>(json_str).value() }
  {}
  [[nodiscard]] constexpr auto Size() const noexcept -> std::size_t;
  [[nodiscard]] auto TotalCards() -> uint32_t
  {
    // If the memoized value doesn't exist, do the work
    if (!this->total_quantity_.has_value()) { memoize_card_quantities(); }
    // Return memoized value
    return this->total_quantity_.value();
  }

  void ExtractGoatbotsInfo(const goatbots::card_defs_map_t &card_defs,
    const goatbots::price_hist_map_t &price_hist) noexcept;
  [[nodiscard]] auto ToJson() const -> std::string;
  [[nodiscard]] auto ToJsonPretty() const -> std::string;
  void Print() const;
  void PrettyPrint() const;
  void FromJson(const std::string &json_str);


private:
  // Helpers

  // The first time anything related to card quantities is needed/called this function is called to avoid doing
  // double work
  void memoize_card_quantities()
  {
    // Parse quantity from string to uint32_t
    // Keep this result in memory for future calls including calls to specific card quantities
    std::vector<uint16_t> card_quantity_tmp(cards_.size(), 0);

    std::transform(
      // Building the vector of quantities is fully parallelizable but apple clang has not implemented std::execution :(
      // std::execution::par,
      this->cards_.begin(),
      this->cards_.end(),
      card_quantity_tmp.begin(),
      [](const mtgo::Card &c) -> uint16_t { return static_cast<uint16_t>(std::stoul(c.quantity_)); });

    // Then sum the quantities in using reduce to prepare for when apple clang implements std::execution in the
    // future...
    this->total_quantity_ = std::reduce(
      card_quantity_tmp.begin(), card_quantity_tmp.end(), 0, [](const auto &a, const auto &b) { return a + b; });

    // Move the vector to the member variable
    this->card_quantity_ = std::move(card_quantity_tmp);
  }
};


constexpr auto Collection::Size() const noexcept -> std::size_t { return cards_.size(); }

void Collection::ExtractGoatbotsInfo(const goatbots::card_defs_map_t &card_defs,
  const goatbots::price_hist_map_t &price_hist) noexcept
{
  for (auto &c : cards_) {
    // Extract set, rarity, and foil from goatbots card definitions
    if (auto res = card_defs.find(c.id_); res != card_defs.end()) {
      c.set_ = res->second.cardset;
      c.rarity_ = res->second.rarity;
      c.foil_ = res->second.foil == 1;
    } else {
      spdlog::warn("Card definition key not found: ID={}", c.id_);
    }
    // Extract price from goatbots price history
    if (auto res = price_hist.find(c.id_); res != price_hist.end()) {
      c.goatbots_price_ = res->second;
    } else {
      spdlog::warn("Price history key not found: ID={}", c.id_);
    }
  }
}
[[nodiscard]] auto Collection::ToJson() const -> std::string { return glz::write_json(cards_); }
[[nodiscard]] auto Collection::ToJsonPretty() const -> std::string
{
  std::string res{};
  glz::write<glz::opts{ .prettify = true }>(cards_, res);
  return res;
}
void Collection::FromJson(const std::string &json_str)
{

  if (auto ec = glz::read_json<std::vector<Card>>(std::ref(cards_), json_str)) {
    spdlog::error("{}", glz::format_error(ec, std::string{}));
  }
}
void Collection::Print() const
{
  for (const auto &c : cards_) {
    fmt::println("{} {}: Goatbots price={}, Scryfall price={}, quantity={}, set={}, foil={}, rarity={}",
      c.id_,
      c.name_,
      c.goatbots_price_,
      c.scryfall_price_,
      c.quantity_,
      c.set_,
      c.foil_,
      c.rarity_);
  }
}

void Collection::PrettyPrint() const
{
  fmt::println("{: <25}{: <23}{: <23}{: <11}{: <8}{: <10}{: <6}\n",
    "Name",
    "Goatbots price [tix]",
    "Scryfall price [tix]",
    "Quantity",
    "Foil",
    "Rarity",
    "Set");
  for (const auto &c : cards_) {
    fmt::println("{: <25}{: <23}{: <23}{: <11}{: <8}{: <10}{: <6}",
      c.name_,
      c.goatbots_price_,
      c.scryfall_price_,
      c.quantity_,
      c.foil_,
      c.rarity_,
      c.set_);
  }
}

}// namespace mtgo
