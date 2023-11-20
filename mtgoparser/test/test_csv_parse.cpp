// NOLINTBEGIN

#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>

#include <fmt/core.h>

#include <mtgoparser/mtgo/csv.hpp>

#include <span>
#include <string>
#include <string_view>
#include <utility>

using Catch::Matchers::ContainsSubstring;


TEST_CASE("mtgo::csv::into_substr_vec")
{
  const std::string test_csv_data =
    R"(id,quantity,name,set,rarity,foil,2023-11-06T083944Z,2023-11-06T115147Z,2023-11-08T084732Z
120020,1,In the Darkness Bind Them,LTC,R,false,[4]0.72;0.1,[8]0.78;-,0.4;0.3
106729,1,Razorverge Thicket,ONE,R,false,[1]1.1;0.9,2.0;2.1,[11]0.9;-
106729,1,Razorverge Thicket,THR,R,false,-;-,[2]2.0;2.1,[0]0.9;-)";

  std::vector<std::string> rows = mtgo::csv::into_substr_vec(test_csv_data, '\n');
  REQUIRE(rows.size() == 4);

  CHECK(rows.at(0) == "id,quantity,name,set,rarity,foil,2023-11-06T083944Z,2023-11-06T115147Z,2023-11-08T084732Z");
  CHECK(rows.at(1) == "120020,1,In the Darkness Bind Them,LTC,R,false,[4]0.72;0.1,[8]0.78;-,0.4;0.3");
  CHECK(rows.at(2) == "106729,1,Razorverge Thicket,ONE,R,false,[1]1.1;0.9,2.0;2.1,[11]0.9;-");
  CHECK(rows.at(3) == "106729,1,Razorverge Thicket,THR,R,false,-;-,[2]2.0;2.1,[0]0.9;-");


  auto headers = mtgo::csv::into_substr_vec(rows[0], ',');
  REQUIRE(headers.size() == 9);

  CHECK(headers.at(0) == "id");
  CHECK(headers.at(1) == "quantity");
  CHECK(headers.at(2) == "name");
  CHECK(headers.at(3) == "set");
  CHECK(headers.at(4) == "rarity");
  CHECK(headers.at(5) == "foil");
  CHECK(headers.at(6) == "2023-11-06T083944Z");
  CHECK(headers.at(7) == "2023-11-06T115147Z");
  CHECK(headers.at(8) == "2023-11-08T084732Z");
}

TEST_CASE("mtgo::csv::str_to_floats")
{
  auto [q0, gb0, sc0] = mtgo::csv::parse_quant_and_prices("[4]0.72;0.1");

  INFO("q: " << q0.value_or(0));
  INFO("gb0: " << gb0.value_or(-1.0f));
  INFO("sc0: " << sc0.value_or(-1.0f));

  REQUIRE(q0.has_value());
  REQUIRE(q0.value() == 4);
  REQUIRE(gb0.has_value());
  REQUIRE(sc0.has_value());
  CHECK(gb0.value() == 0.72f);
  CHECK(sc0.value() == 0.1f);

  auto [q01, gb01, sc01] = mtgo::csv::parse_quant_and_prices("0.002;12.1");

  CHECK_FALSE(q01.has_value());
  CHECK(gb01.has_value());
  CHECK(sc01.has_value());
  CHECK(gb01.value() == 0.002f);
  CHECK(sc01.value() == 12.1f);

  auto [q1, gb1, sc1] = mtgo::csv::parse_quant_and_prices("[9]0.72;-");

  CHECK(q1.has_value());
  CHECK(gb1.has_value());
  CHECK_FALSE(sc1.has_value());
  CHECK(q1.value() == 9);
  CHECK(gb1.value() == 0.72f);


  auto [q2, gb2, sc2] = mtgo::csv::parse_quant_and_prices("-;0.1");

  CHECK_FALSE(q2.has_value());
  CHECK_FALSE(gb2.has_value());
  CHECK(sc2.has_value());
  CHECK(sc2.value() == 0.1f);

  auto [q3, gb3, sc3] = mtgo::csv::parse_quant_and_prices("-;-");

  CHECK_FALSE(q3.has_value());
  CHECK_FALSE(gb3.has_value());
  CHECK_FALSE(sc3.has_value());

  // With more than two values
  auto [q4, gb4, b4] = mtgo::csv::parse_quant_and_prices("0.72;0.1;0.2");

  CHECK_FALSE(q4.has_value());
  CHECK(gb4.has_value());
  CHECK(b4.has_value());
  CHECK(gb4.value() == 0.72f);
  CHECK(b4.value() == 0.1f);

  // With integer values
  auto [q5, gb5, sc5] = mtgo::csv::parse_quant_and_prices("[11]1;2");

  CHECK(q5.has_value());
  CHECK(gb5.has_value());
  CHECK(sc5.has_value());
  CHECK(q5.value() == 11);
  CHECK(gb5.value() == 1.0f);
  CHECK(sc5.value() == 2.0f);
}


TEST_CASE("mtgo::csv::into_substr_vec & mtgo::csv::str_to_floats")
{
  const std::string test_csv_data =
    R"(id,quantity,name,set,rarity,foil,2023-11-06T083944Z,2023-11-06T115147Z,2023-11-08T084732Z
120020,1,In the Darkness Bind Them,LTC,R,false,[4]0.72;0.1,[8]0.78;-,0.4;0.3
106729,1,Razorverge Thicket,ONE,R,false,[1]1.1;0.9,2.0;2.1,[11]0.9;-
106729,1,Razorverge Thicket,THR,R,false,-;-,[2]2.0;2.1,[0]0.9;-)";

  std::vector<std::string> rows = mtgo::csv::into_substr_vec(test_csv_data, '\n');
  REQUIRE(rows.size() == 4);

  auto headers = mtgo::csv::into_substr_vec(rows[0], ',');
  REQUIRE(headers.size() == 9);

  auto row1 = mtgo::csv::into_substr_vec(rows.at(1), ',');
  REQUIRE(row1.size() == 9);

  auto row2 = mtgo::csv::into_substr_vec(rows.at(2), ',');
  REQUIRE(row2.size() == 9);

  auto row3 = mtgo::csv::into_substr_vec(rows.at(3), ',');
  REQUIRE(row3.size() == 9);

  SECTION("Parse quantity, gb price, and scryfall price")
  {
    SECTION("Row 1")
    {
      auto [q0, gb0, sc0] = mtgo::csv::parse_quant_and_prices(row1.at(6));

      CHECK(q0.has_value());
      CHECK(gb0.has_value());
      CHECK(sc0.has_value());
      CHECK(q0.value() == 4);
      CHECK(gb0.value() == 0.72f);
      CHECK(sc0.value() == 0.1f);

      auto [q1, gb1, sc1] = mtgo::csv::parse_quant_and_prices(row1.at(7));

      CHECK(q1.has_value());
      CHECK(gb1.has_value());
      CHECK_FALSE(sc1.has_value());
      CHECK(q1.value() == 8);
      CHECK(gb1.value() == 0.78f);

      auto [q2, a2, b2] = mtgo::csv::parse_quant_and_prices(row1.at(8));

      CHECK_FALSE(q2.has_value());
      CHECK(a2.has_value());
      CHECK(b2.has_value());
      CHECK(a2.value() == 0.4f);
      CHECK(b2.value() == 0.3f);
    }

    SECTION("Row 2")
    {
      for (std::size_t i = 6; i < 9; ++i) {
        auto [q, gb, sc] = mtgo::csv::parse_quant_and_prices(row2.at(i));
        if (i == 6) {
          CHECK(q.has_value());
          CHECK(gb.has_value());
          CHECK(sc.has_value());
          CHECK(q.value() == 1);
          CHECK(gb.value() == 1.1f);
          CHECK(sc.value() == 0.9f);
        } else if (i == 7) {
          CHECK_FALSE(q.has_value());
          CHECK(gb.has_value());
          CHECK(sc.has_value());
          CHECK(gb.value() == 2.0f);
          CHECK(sc.value() == 2.1f);
        } else if (i == 8) {
          CHECK(q.has_value());
          CHECK(gb.has_value());
          CHECK_FALSE(sc.has_value());
          CHECK(q.value() == 11);
          CHECK(gb.value() == 0.9f);
        }
      }
    }
  }
}

TEST_CASE("mtgo::csv::floats_from_span")
{
  SECTION("Simple")
  {
    std::vector<std::string> row{ "[1]0.72;0.1", "0.78;-", "[11]0.4;0.3" };

    auto q_gb_sc = mtgo::csv::quant_and_prices_from_span(std::span(row));
    REQUIRE(q_gb_sc.size() == 3);
  }

  SECTION("On CSV Data with mtgo::csv::into_substr_vec")
  {
    const std::string test_csv_data =
      R"(id,quantity,name,set,rarity,foil,2023-11-06T083944Z,2023-11-06T115147Z,2023-11-08T084732Z
120020,1,In the Darkness Bind Them,LTC,R,false,[4]0.72;0.1,[8]0.78;-,0.4;0.3
106729,1,Razorverge Thicket,ONE,R,false,[1]1.1;0.9,2.0;2.1,[11]0.9;-
106729,1,Razorverge Thicket,THR,R,false,-;-,[2]2.0;2.1,[0]0.9;-)";

    std::vector<std::string> rows = mtgo::csv::into_substr_vec(test_csv_data, '\n');
    REQUIRE(rows.size() == 4);
    auto headers = mtgo::csv::into_substr_vec(rows[0], ',');
    REQUIRE(headers.size() == 9);
    auto row1 = mtgo::csv::into_substr_vec(rows.at(1), ',');
    REQUIRE(row1.size() == 9);

    auto q_gb_sc = mtgo::csv::quant_and_prices_from_span(std::span(row1).subspan(6));
    REQUIRE(q_gb_sc.size() == 3);

    CHECK(std::get<0>(q_gb_sc.at(0)).has_value());
    CHECK(std::get<1>(q_gb_sc.at(0)).has_value());
    CHECK(std::get<2>(q_gb_sc.at(0)).has_value());
    CHECK(std::get<0>(q_gb_sc.at(0)).value() == 4);
    CHECK(std::get<1>(q_gb_sc.at(0)).value() == 0.72f);
    CHECK(std::get<2>(q_gb_sc.at(0)).value() == 0.1f);

    CHECK(std::get<0>(q_gb_sc.at(1)).has_value());
    CHECK(std::get<1>(q_gb_sc.at(1)).has_value());
    CHECK_FALSE(std::get<2>(q_gb_sc.at(1)).has_value());

    CHECK(std::get<0>(q_gb_sc.at(1)).value() == 8);
    CHECK(std::get<1>(q_gb_sc.at(1)).value() == 0.78f);

    CHECK_FALSE(std::get<0>(q_gb_sc.at(2)).has_value());
    CHECK(std::get<1>(q_gb_sc.at(2)).has_value());
    CHECK(std::get<2>(q_gb_sc.at(2)).has_value());
    CHECK(std::get<1>(q_gb_sc.at(2)).value() == 0.4f);
    CHECK(std::get<2>(q_gb_sc.at(2)).value() == 0.3f);
  }

  SECTION("Parse CSV into row data and back into CSV string")
  {
    const std::string test_csv_data =
      R"(id,quantity,name,set,rarity,foil,2023-11-06T083944Z,2023-11-06T115147Z,2023-11-08T084732Z
120020,1,In the Darkness Bind Them,LTC,R,false,[4]0.72;0.1,[8]0.78;-,0.4;0.3)";

    std::vector<std::string> rows = mtgo::csv::into_substr_vec(test_csv_data, '\n');
    REQUIRE(rows.size() == 2);
    auto headers = mtgo::csv::into_substr_vec(rows[0], ',');
    REQUIRE(headers.size() == 9);
    auto row1 = mtgo::csv::into_substr_vec(rows.at(1), ',');
    REQUIRE(row1.size() == 9);

    auto q_gb_sc = mtgo::csv::quant_and_prices_from_span(std::span(row1).subspan(6));
    REQUIRE(q_gb_sc.size() == 3);

    std::string csv_str = fmt::format("{},{},{},{},{},{},{},{},{}\n{},{},{},{},{},{}",
      headers.at(0),
      headers.at(1),
      headers.at(2),
      headers.at(3),
      headers.at(4),
      headers.at(5),
      headers.at(6),
      headers.at(7),
      headers.at(8),
      row1.at(0),
      row1.at(1),
      row1.at(2),
      row1.at(3),
      row1.at(4),
      row1.at(5));

    INFO("csv_str formatted before adding floats:\n" << csv_str);

    for (const auto &[q, gb, sc] : q_gb_sc) {

      std::string quantity = q.has_value() ? fmt::format("[{}]", q.value()) : "";
      std::string gb_str = gb.has_value() ? fmt::format("{}", gb.value()) : "-";
      std::string b_str = sc.has_value() ? fmt::format("{}", sc.value()) : "-";
      csv_str += fmt::format(",{}{};{}", quantity, gb_str, b_str);
    }

    INFO("csv_str formatting complete with floats added:\n" << csv_str);

    CHECK(csv_str == test_csv_data);
  }
}

// NOLINTEND