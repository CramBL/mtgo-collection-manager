#pragma once

#include <concepts>
#include <glaze/glaze.hpp>
#include <string>
#include <string_view>

namespace mtgo {

struct Card
{
  std::string id_;
  std::string quantity_;
  std::string name_;
  std::string set_;
  std::string rarity_;
  bool foil_;
  double goatbots_price_;
  double scryfall_price_;


  // Default constructor
  // Note: some builds raises false positives in static analysis when simply declared as `Card() = default` )
  [[nodiscard]] explicit Card(std::string id = "",
    std::string quantity = "",
    std::string name = "",
    std::string set = "",
    std::string rarity = "",
    bool foil = false,
    double goatbots_price = 0,
    double scryfall_price = 0) noexcept
    : id_{ id }, quantity_{ quantity }, name_{ name }, set_{ set }, rarity_{ rarity }, foil_{ foil },
      goatbots_price_{ goatbots_price }, scryfall_price_{ scryfall_price }
  {}

  // Partially parametrised constructor used to construct a card from MTGO .dek XML
  [[nodiscard]] explicit Card(const char *id,
    const char *quantity,
    const char *name,
    const char *set = "",
    const char *rarity = "",
    bool foil = false,
    double goatbots_price = 0,
    double scryfall_price = 0) noexcept
    : id_{ id }, quantity_{ quantity }, name_{ name }, set_{ set }, rarity_{ rarity }, foil_{ foil },
      goatbots_price_{ goatbots_price }, scryfall_price_{ scryfall_price }
  {}

  // SAFETY: The string_views used for construction has to outlive the constructed instance
  // Constructor with string_view beware of lifetimes
  [[nodiscard]] explicit Card(std::string_view id,
    std::string_view quantity,
    std::string_view name,
    std::string_view set,
    std::string_view rarity,
    bool foil = false,
    double goatbots_price = 0,
    double scryfall_price = 0) noexcept
    : id_{ id }, quantity_{ quantity }, name_{ name }, set_{ set }, rarity_{ rarity }, foil_{ foil },
      goatbots_price_{ goatbots_price }, scryfall_price_{ scryfall_price }
  {}

  // Templated constructor
  template<typename T>
  requires std::convertible_to<T, std::string>
  explicit Card(T id,
    T quantity,
    T name,
    T set,
    T rarity,
    bool foil = false,
    double goatbots_price = 0,
    double scryfall_price = 0) noexcept
    : id_{ id }, quantity_{ quantity }, name_{ name }, set_{ set }, rarity_{ rarity }, foil_{ foil },
      goatbots_price_{ goatbots_price }, scryfall_price_{ scryfall_price }
  {}

  // Move constructor
  [[nodiscard]] Card(Card &&other) noexcept
    : id_(std::move(other.id_)), quantity_(std::move(other.quantity_)), name_(std::move(other.name_)),
      set_(std::move(other.set_)), rarity_(std::move(other.rarity_)), foil_(other.foil_),
      goatbots_price_(other.goatbots_price_), scryfall_price_(other.scryfall_price_)
  {}

  // Move assignment operator
  [[nodiscard]] Card &operator=(Card &&other) noexcept
  {
    if (this != &other) {
      id_ = std::move(other.id_);
      quantity_ = std::move(other.quantity_);
      name_ = std::move(other.name_);
      set_ = std::move(other.set_);
      rarity_ = std::move(other.rarity_);
      foil_ = other.foil_;
      goatbots_price_ = other.goatbots_price_;
      scryfall_price_ = other.scryfall_price_;
    }

    return *this;
  }
};
}// namespace mtgo

template<> struct glz::meta<mtgo::Card>
{
  using T = mtgo::Card;
  static constexpr auto value = object("id",
    &T::id_,
    "quantity",
    &T::quantity_,
    "name",
    &T::name_,
    "set",
    &T::set_,
    "rarity",
    &T::rarity_,
    "foil",
    &T::foil_,
    "goatbots_price",
    &T::goatbots_price_,
    "scryfall_price",
    &T::scryfall_price_);
};