// NOLINTBEGIN

#include "mtgoparser/clap/option.hpp"
#include "mtgoparser/io.hpp"
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>
#include <fmt/core.h>
#include <mtgoparser/clap.hpp>
#include <mtgoparser/io.hpp>
#include <mtgoparser/io/gzip.hpp>
#include <mtgoparser/mtg.hpp>
#include <mtgoparser/mtgo/card.hpp>
#include <mtgoparser/util.hpp>


#include <regex>
#include <string_view>
#include <utility>

using Catch::Matchers::ContainsSubstring;

using clap::Opt::Flag;
using clap::Opt::NeedValue;

constinit auto static_clap = clap::Clap<1, 0>(clap::OptionArray<1>(clap::Option("--version", Flag)));


TEST_CASE("Test basic CLAP")
{
  char argv0[] = "mtgo_preprocessor";
  char argv1[] = "--version";

  char *argv[] = { argv0, argv1 };
  int argc = 2;

  std::vector<std::string_view> arg_vec{ argv + 1, argv + argc };

  SECTION("Dynamically initialized - Show version")
  {
    auto clap = clap::Clap<1, 0>(clap::OptionArray<1>(clap::Option("--version", Flag)));
    fmt::print("Options are:\n");
    clap.PrintOptions();

    CHECK(clap.Parse(arg_vec) == 0);
    fmt::print("Arguments are:\n");
    clap.PrintArgs();
  }

  SECTION("Static initialized - show version")
  {
    fmt::print("Parsing arguments with constinit Clap\n");
    CHECK(static_clap.Parse(arg_vec) == 0);
    fmt::print("Arguments are:\n");
    static_clap.PrintArgs();
  }

  SECTION("Alias version cmd - Show version")
  {
    auto clap_alias_version = clap::Clap<1, 0>(clap::OptionArray<1>(clap::Option("--version", Flag, "-V")));

    CHECK(clap_alias_version.Parse(arg_vec) == 0);

    fmt::print("Arguments are:\n");
    clap_alias_version.PrintArgs();

    CHECK(clap_alias_version.FlagSet("--version"));
    CHECK(clap_alias_version.FlagSet("-V"));
  }
}

TEST_CASE("Test CLAP with options and values")
{
  char argv0[] = "mtgo_preprocessor";
  char arg_version[] = "--version";
  char arg_save_as[] = "--save-as";
  char arg_save_as_val[] = "saved.txt";

  SECTION("test save as option value")
  {
    char *argv[] = { argv0, arg_save_as, arg_save_as_val };
    int argc = 3;

    std::vector<std::string_view> arg_vec{ argv + 1, argv + argc };

    auto clap = clap::Clap<2, 0>(
      clap::OptionArray<2>(clap::Option("--version", Flag, "-V"), clap::Option("--save-as", NeedValue, "-s")));


    CHECK(clap.Parse(arg_vec) == 0);
    fmt::print("Got args:\n");
    clap.PrintArgs();

    CHECK(clap.OptionValue("--save-as").value() == arg_save_as_val);
    CHECK(clap.OptionValue("-s").value() == arg_save_as_val);
    CHECK(clap.FlagSet("--version") == false);
    CHECK(clap.FlagSet("-V") == false);
  }

  SECTION("Argument validation catches errors")
  {
    constexpr auto version_option = clap::Option("--version", Flag, "-V");
    constexpr auto save_as_option = clap::Option("--save-as", NeedValue, "-s");
    constexpr auto opt_arr = clap::OptionArray<2>(version_option, save_as_option);
    auto clap = clap::Clap<2, 0>(opt_arr);


    SECTION("Missing option value - end of args")
    {
      char *argv[] = { argv0, arg_save_as };
      int argc = 2;
      std::vector<std::string_view> arg_vec{ argv + 1, argv + argc };


      fmt::print("Got args:\n");
      fmt::print("Should fail as --save-as doesn't have a value provided\n");
      CHECK(clap.Parse(arg_vec) != 0);
    }

    SECTION("Missing option value - next option instead of value")
    {
      char *argv[] = { argv0, arg_save_as, arg_version };
      int argc = 3;
      std::vector<std::string_view> arg_vec{ argv + 1, argv + argc };

      fmt::print("Got args:\n");
      fmt::print(
        "Should fail as --save-as doesn't have a value provided, instead it's followed by the --version option\n");
      CHECK(clap.Parse(arg_vec) != 0);
    }
  }
}

TEST_CASE("MTGO card - Initialize and use of")
{
  SECTION("Initialize")
  {
    // Test constructors, assignments, initializations with different types
    unsigned short int id_1 = 1;
    mtgo::Card mtgo_card = mtgo::Card(id_1, util::sv_to_uint<uint16_t>("1").value(), "name", "set", "Common");
    CHECK(mtgo_card.id_ == 1);
    CHECK(mtgo_card.quantity_ == 1);
    CHECK(mtgo_card.name_ == "name");
    CHECK(mtgo_card.set_ == "set");
    CHECK(mtgo_card.rarity_ == mtg::Rarity::Common);
    CHECK(mtgo_card.foil_ == false);
    CHECK(mtgo_card.goatbots_price_ == 0);
    REQUIRE(mtgo_card.scryfall_price_.has_value() == false);
    REQUIRE(mtgo_card.scryfall_price_ == std::nullopt);

    unsigned int id2 = 1;
    mtgo::Card mtgo_card2 =
      mtgo::Card(id2, util::sv_to_uint<uint16_t>("1").value(), "name", "set", "C", true, 1.0f, 2.0f);
    CHECK(mtgo_card2.id_ == 1);
    CHECK(mtgo_card2.quantity_ == 1);
    CHECK(mtgo_card2.name_ == "name");
    CHECK(mtgo_card2.set_ == "set");
    CHECK(mtgo_card2.rarity_ == mtg::Rarity::Common);
    CHECK(mtgo_card2.foil_ == true);
    CHECK(mtgo_card2.goatbots_price_ == 1.0f);
    REQUIRE(mtgo_card2.scryfall_price_.has_value());
    REQUIRE(mtgo_card2.scryfall_price_.value() == 2.0f);

    CHECK(mtgo_card != mtgo_card2);

    // Check initialization from string_view
    uint32_t id = 1;
    std::string_view quantity = "1";
    std::string_view name = "name";
    std::string_view set = "set";
    std::string_view rarity = "common";
    mtgo::Card mtgo_card3 = mtgo::Card(id, util::sv_to_uint<uint16_t>(quantity).value(), name, set, rarity);

    // check equality with mtgo_card2
    CHECK(mtgo_card3 != mtgo_card2);
    CHECK(mtgo_card3 == mtgo_card);

    // Check initialization from string
    std::string name_str = "name";
    std::string set_str = "set";
    std::string rarity_str = "COMMON";
    uint32_t id4 = 1;
    uint16_t quant4 = 1;
    mtgo::Card mtgo_card4 = mtgo::Card(id4, quant4, name_str, set_str, rarity_str);

    // check equality with mtgo_card
    CHECK(mtgo_card4 == mtgo_card);
    CHECK(mtgo_card4 == mtgo_card3);
    CHECK(mtgo_card4 != mtgo_card2);
  }

  SECTION("Card Move semantics")
  {
    // Test move constructors and move assignment

    uint16_t ids = 1;
    uint16_t quantities = 1;
    mtgo::Card mtgo_card = mtgo::Card(ids, quantities, "name", "set", "common", true, 1.0f, 2.0f);
    mtgo::Card mtgo_card2 = mtgo::Card(ids, quantities, "name", "set", "common", true, 1.0f, 2.0f);

    // Move constructor
    mtgo::Card mtgo_card3(std::move(mtgo_card));
    CHECK(mtgo_card3 == mtgo_card2);
    // Check that mtgo_card is now invalid (commented out as it triggered warning in CI)
    // CHECK(mtgo_card.id_ == "");// Access of moved value

    // Move assignment
    uint16_t id_tmp = 2;
    auto mtgo_card_tmp = mtgo::Card(id_tmp, quantities, "name", "set", "common", true, 1.0f, 2.0f);
    mtgo_card3 = std::move(mtgo_card_tmp);
    CHECK(mtgo_card3 != mtgo_card2);// ID should differ
    // Check that mtgo_card_tmp is now invalid (commented out as it triggered warning in CI)
    // CHECK(mtgo_card_tmp.id_ == ""); // Access of moved value (compiler warning)
  }
}

TEST_CASE("Command struct")
{
  // Command with no aliases
  constexpr clap::Command cmd0{ "my-cmd", false };
  CHECK(cmd0.name_ == "my-cmd");
  CHECK(cmd0.is_flag_ == false);

  // with alias
  constexpr clap::Command cmd1{ "my-cmd1", false };
  CHECK(cmd1.name_ == "my-cmd1");
  CHECK(cmd1.is_flag_ == false);

  // With multiple aliases
  constexpr clap::Command cmd2{ "my-cmd2", true };
  CHECK(cmd2.name_ == "my-cmd2");
  CHECK(cmd2.is_flag_ == true);

  // They can fit in same cmd array
  constexpr std::array<clap::Command, 3> cmd_arr = { cmd0, cmd1, cmd2 };
  REQUIRE(cmd_arr.at(0).name_ == cmd0.name_);
  CHECK(cmd0.is_flag_ == false);

  REQUIRE(cmd_arr.at(2).name_ == "my-cmd2");
  REQUIRE(cmd_arr.at(2).is_flag_ == true);

  constexpr clap::CommandArray<3> my_cmd_arr{ cmd0, cmd1, cmd2 };
  constexpr auto arr_sz = my_cmd_arr.size();// Circumvent CPP check warning: [knownConditionTrueFalse]
  REQUIRE(arr_sz == 3);
  CHECK(my_cmd_arr.find("my-cmd2").has_value());
  CHECK(my_cmd_arr.find("my-cmd1").value().name_ == "my-cmd1");
  CHECK(my_cmd_arr.find("my-cmd1").value().is_flag_ == false);
}

TEST_CASE("Option struct")
{
  constexpr clap::Option opt{ "--my-option", Flag };
  constexpr clap::Option opt_w_alias("--my-option", Flag, "--my-alias");

  constexpr bool opt_has_alias = opt.has_alias();
  REQUIRE(opt_has_alias == false);

  constexpr bool opt_w_alias_has_alias = opt_w_alias.has_alias();
  REQUIRE(opt_w_alias_has_alias == true);

  constexpr clap::OptionArray<2> opt_arr{ opt, opt_w_alias };

  constexpr auto arr_sz = opt_arr.size();
  CHECK(arr_sz == 2);

  CHECK(opt_arr.find("--my-option").has_value() == true);
  CHECK(opt_arr.find("--my-alias").has_value() == true);

  auto found_opt = opt_arr.find("--my-alias");
  REQUIRE(found_opt.has_value() == true);
  CHECK(found_opt.value().name_ == "--my-option");
}

TEST_CASE("Parse state_log.toml")
{
  const auto path_state_log = "../../../test/test-data/mtgogetter-out/state_log.toml";
  auto state_log = io_util::read_state_log(path_state_log);
  std::string_view title = state_log["title"].value_or("");

  INFO("state_log has title: " << title);
  CHECK(title == "log for MTGO Getter state, such as updated_at timestamps");

  // Check goatbots values
  SECTION("Goatbots state_log data")
  {
    std::optional<toml::date_time> card_defs_updated_at =
      state_log["goatbots"]["card_definitions_updated_at"].value<toml::date_time>();
    REQUIRE(card_defs_updated_at.has_value());

    INFO("state_log.goatbots.card_definitions_updated_at: " << card_defs_updated_at.value());
    CHECK(
      card_defs_updated_at.value() == toml::date_time{ { 2023, 10, 21 }, { 22, 29, 53 }, {} });// "2023-10-21T22:29:53Z"

    std::optional<toml::date_time> prices_updated_at =
      state_log["goatbots"]["prices_updated_at"].value<toml::date_time>();
    REQUIRE(prices_updated_at.has_value());
    INFO("state_log.goatbots.prices_updated_at: " << prices_updated_at.value());
    CHECK(
      prices_updated_at.value() == toml::date_time{ { 2023, 10, 14 }, { 15, 24, 21 }, {} });// "2023-10-14T15:24:21Z"
  }

  SECTION("Scryfall state_log data")
  {
    // check scryfall values
    std::optional<toml::date_time> bulk_updated_at =
      state_log["scryfall"]["bulk_data_updated_at"].value<toml::date_time>();
    REQUIRE(bulk_updated_at.has_value());
    INFO("state_log.scryfall.bulk_data_updated_at: " << bulk_updated_at.value());
    CHECK(bulk_updated_at.value() == toml::date_time{ { 1970, 1, 1 }, { 0, 0, 0 }, {} });//"1970-01-01T00:00:00Z"

    // check scryfall next released set
    std::string_view next_released_set_name = state_log["scryfall"]["next_released_mtgo_set"]["name"].value_or("");
    CHECK(next_released_set_name == "The Lost Caverns of Ixalan");

    std::string_view next_released_set_date_str =
      state_log["scryfall"]["next_released_mtgo_set"]["released_at"].value_or("");
    CHECK(next_released_set_date_str == "2023-12-11");

    std::string_view next_released_set_mtgo_code =
      state_log["scryfall"]["next_released_mtgo_set"]["mtgo_code"].value_or("");
    CHECK(next_released_set_mtgo_code == "lci");
  }

  SECTION("Write to state_log")
  {
    // Pretend we found the mtgo_code in the card defs, now we should empty the "next_released_mtgo_set" fields
    toml::value<std::string> *name = state_log["scryfall"]["next_released_mtgo_set"]["name"].as_string();
    *name = "";
    toml::value<std::string> *released_at = state_log["scryfall"]["next_released_mtgo_set"]["released_at"].as_string();
    *released_at = "";
    toml::value<std::string> *mtgo_code = state_log["scryfall"]["next_released_mtgo_set"]["mtgo_code"].as_string();
    *mtgo_code = "";

    std::string name_str = state_log["scryfall"]["next_released_mtgo_set"]["name"].value_or("error");
    CHECK(name_str == "");

    std::string released_at_str = state_log["scryfall"]["next_released_mtgo_set"]["released_at"].value_or("error");
    CHECK(released_at_str == "");

    std::string mtgo_code_str = state_log["scryfall"]["next_released_mtgo_set"]["mtgo_code"].value_or("error");
    CHECK(mtgo_code_str == "");

    INFO("State log:\n" << state_log << '\n');

    SECTION("TOML File operations")
    {
      std::ofstream test_state_log_file("test_tmp_state_log.toml");
      REQUIRE(test_state_log_file.is_open());

      if (test_state_log_file.is_open()) {
        INFO("test_tmp_state_log.toml opened");
        test_state_log_file << state_log;
        test_state_log_file.close();
      } else {
        FAIL("Opening file for writing failed.");
      }


      std::ifstream newly_open_test_state_log_file("test_tmp_state_log.toml");
      REQUIRE(newly_open_test_state_log_file.is_open());
      if (newly_open_test_state_log_file.is_open()) {
        std::string line{};
        while (std::getline(newly_open_test_state_log_file, line)) { INFO("Line from file: " << line); }
        newly_open_test_state_log_file.close();
      } else {
        FAIL("Opening file for reading failed.");
      }
    }
  }
}

TEST_CASE("io_util::save_with_timestamp")
{
  // Regular expression pattern for ISO 8601 timestamp without sub-second precision and without colons.
  // e.g. 2023-11-05T152700Z
  std::regex iso8601_pattern(R"(\d{4}-\d{2}-\d{2}T\d{2}\d{2}\d{2}Z)");

  SECTION("Check regex validation")
  {
    // Check the ISO 8601 timestamp regex is okay
    std::string timestamp_ok_str = "This is a string containing an ISO 8601 timestamp 2023-11-05T092102Z.";

    // Check that a non-IS0 8601 timestamp is not matched
    std::string timestamp_not_ok_str = "This is a string containing a non-ISO 8601 timestamp 2023-11-05T09:21:02.";
    // And more variations that should not match
    std::string timestamp_not_ok_str2 = "This string lacks a timestamp.";
    std::string timestamp_not_ok_str3 = "This string's timestamp is missing the Z at the end 2023-11-05T09:21:02";
    std::string timestamp_not_ok_str4 = "This string's timestamp is missing the T in the middle 2023-11-05 09:21:02Z";
    std::string timestamp_not_ok_str5 = "This string's timestamp is missing the T and Z 2023-11-05 09:21:02";
    std::string timestamp_not_ok_colons_str =
      "This is a string containing an ISO 8601 timestamp BUT WITH COLONS 2023-11-05T09:21:02Z.";


    // Require that the string matches the pattern
    REQUIRE(std::regex_search(timestamp_ok_str, iso8601_pattern));

    // Check that the string does not match the pattern
    CHECK_FALSE(std::regex_search(timestamp_not_ok_str, iso8601_pattern));
    CHECK_FALSE(std::regex_search(timestamp_not_ok_str2, iso8601_pattern));
    CHECK_FALSE(std::regex_search(timestamp_not_ok_str3, iso8601_pattern));
    CHECK_FALSE(std::regex_search(timestamp_not_ok_str4, iso8601_pattern));
    CHECK_FALSE(std::regex_search(timestamp_not_ok_str5, iso8601_pattern));
    CHECK_FALSE(std::regex_search(timestamp_not_ok_colons_str, iso8601_pattern));
  }


  SECTION("Save to current directory")
  {
    // Save a file "test_tmp_file_<timestamp>.txt" to the current directory
    const std::string fname{ "TEST_TMP_FILE" };
    const std::string test_file_contents = "test file contents";
    const std::string extension = "txt";

    // Make it a path
    const std::filesystem::path test_file_path = fname;

    // Save file
    auto res = io_util::save_with_timestamp(test_file_contents, test_file_path, extension);

    if (res.has_error()) { FAIL("Error saving file: " << res.error()); }

    // Check that file was saved
    REQUIRE(res.has_value());
    auto created_file_path = res.value();

    // Check that file name contains the original file name
    auto created_filename = created_file_path.filename().string();
    INFO("created_filename: " << created_filename);
    INFO("At path: " << created_file_path.string());

    CHECK_THAT(created_filename, ContainsSubstring(fname));
    // Check that file name contains the ISO 8601 timestamp
    CHECK(std::regex_search(created_filename, iso8601_pattern));

    // Check that file exists
    CHECK(std::filesystem::exists(created_file_path));

    {
      // Check that file contents are correct
      std::ifstream test_file(created_file_path);
      std::string test_file_contents_read;
      std::getline(test_file, test_file_contents_read);
      CHECK(test_file_contents_read == test_file_contents);
    }

    // Clean up by removing the file
    std::filesystem::remove(created_file_path);
    CHECK(std::filesystem::exists(test_file_path) == false);
  }

  SECTION("Save to sub-directory")
  {
    // Save a file "test_tmp_file_<timestamp>.txt" to the sub-directory "test_dir"
    const std::string fname{ "test_tmp_file" };
    const std::string extension = "json";
    const std::string test_file_contents =
      R"([{"id":110465,"quantity":1,"name":"Tranquil Cove","set":"MOM","rarity":"C","foil":true,"goatbots_price":0.004}])";
    const std::filesystem::path test_file_path = "test_dir/" + fname;

    // Save file
    auto res = io_util::save_with_timestamp(test_file_contents, test_file_path, extension);

    // Check that file was saved
    REQUIRE(res.has_value());
    auto created_file_path = res.value();

    // Check that file name contains the original file name
    auto created_filename = created_file_path.filename().string();
    INFO("created_filename: " << created_filename);
    INFO("At path: " << created_file_path.string());

    CHECK_THAT(created_filename, ContainsSubstring(fname));
    // Check that file name contains the ISO 8601 timestamp
    CHECK(std::regex_search(created_filename, iso8601_pattern));

    // Check that file exists
    CHECK(std::filesystem::exists(created_file_path));

    {
      // Check that file contents are correct
      std::ifstream test_file(created_file_path);
      std::string test_file_contents_read;
      std::getline(test_file, test_file_contents_read);
      CHECK(test_file_contents_read == test_file_contents);
    }

    // Clean up by removing the file and directory
    std::filesystem::remove_all("test_dir");
  }

  SECTION("Save to adjacent directory")
  {
    // Save a file "test_tmp_file_<timestamp>.txt" to a directory adjacent to the current directory "test_adjacent_dir"
    const std::string fname{ "test_tmp_file" };
    const std::string extension = "json";
    const std::string test_file_contents =
      R"([{"id":110465,"quantity":1,"name":"Tranquil Cove","set":"MOM","rarity":"C","foil":true,"goatbots_price":0.004}])";
    const std::filesystem::path test_file_path = "../test_adjacent_dir/" + fname;

    // Save file
    auto res = io_util::save_with_timestamp(test_file_contents, test_file_path, extension);

    // Check that file was saved
    REQUIRE(res.has_value());
    auto created_file_path = res.value();

    // Check that file name contains the original file name
    auto created_filename = created_file_path.filename().string();
    INFO("created_filename: " << created_filename);
    INFO("At path: " << created_file_path.string());

    CHECK_THAT(created_filename, ContainsSubstring(fname));
    // Check that file name contains the ISO 8601 timestamp
    CHECK(std::regex_search(created_filename, iso8601_pattern));

    // Check that file exists
    CHECK(std::filesystem::exists(created_file_path));

    {
      // Check that file contents are correct
      std::ifstream test_file(created_file_path);
      std::string test_file_contents_read;
      std::getline(test_file, test_file_contents_read);
      CHECK(test_file_contents_read == test_file_contents);
    }

    // Clean up by removing the file and directory
    std::filesystem::remove(created_file_path);
  }
}

TEST_CASE("io_util::is_files_equal")
{
  SECTION("Check two files are identical")
  {
    SECTION("Compare two identical files in current directory")
    {
      // Create two identical files
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };

      // save files to current directory
      {
        std::ofstream test_fileA(fnameA);
        std::ofstream test_fileB(fnameB);

        // write some data to the files
        test_fileA << "test file contents";
        test_fileB << "test file contents";
      }

      // Check that files are equal
      auto cmp_res = io_util::is_files_equal(fnameA, fnameB);
      CHECK(cmp_res.has_value());
      CHECK(cmp_res.value());

      // Clean up by removing the files
      std::filesystem::remove(fnameA);
      std::filesystem::remove(fnameB);
    }

    SECTION("Compare two identical files in different subdirectories")
    {
      // Create two identical files in two different directories
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };
      const std::string dirA{ "test_ident_dirA" };
      const std::string dirB{ "test_ident_dirB" };
      const std::string test_file_contents =
        R"([{"id":110465,"quantity":1,"name":"Tranquil Cove","set":"MOM","rarity":"C","foil":true,"goatbots_price":0.004}])";

      // Make the directories
      std::filesystem::create_directory(dirA);
      std::filesystem::create_directory(dirB);

      // Paths to the files
      const std::filesystem::path pathA = dirA + "/" + fnameA;
      const std::filesystem::path pathB = dirB + "/" + fnameB;

      // Save the files
      {
        std::ofstream test_fileA(pathA);
        std::ofstream test_fileB(pathB);

        // write some data to the files
        test_fileA << test_file_contents;
        test_fileB << test_file_contents;
      }

      // Check that files are equal
      auto cmp_res = io_util::is_files_equal(pathA, pathB);
      CHECK(cmp_res.has_value());
      CHECK(cmp_res.value());

      // Clean up by removing the files and directories
      std::filesystem::remove_all(dirA);
      std::filesystem::remove_all(dirB);
    }

    SECTION("Compare two identical files - one in parent directory, one in current directory")
    {
      // Create two identical files in two different directories
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };
      const std::string dirA{ "test_ident_dirA" };

      const std::string contents{ "contents'~~ _[@]... log. test content///DEe`2èøøæ" };

      // Make the directories
      std::filesystem::create_directory(dirA);

      // Paths to the files
      const std::filesystem::path pathA = dirA + "/" + fnameA;
      const std::filesystem::path pathB = fnameB;

      // Save the files
      {
        std::ofstream test_fileA(pathA);
        std::ofstream test_fileB(pathB);

        // write some data to the files
        test_fileA << contents;
        test_fileB << contents;
      }

      // Check that files are equal
      auto cmp_res = io_util::is_files_equal(pathA, pathB);
      CHECK(cmp_res.has_value());
      CHECK(cmp_res.value());

      // Clean up by removing the files and directories
      std::filesystem::remove_all(dirA);
      std::filesystem::remove(fnameB);
    }
  }

  SECTION("Check two files are NOT identical")
  {
    SECTION("Compare two files in the current directory with different contents")
    {
      // Create two files
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };

      // save files to current directory
      {
        std::ofstream test_fileA(fnameA);
        std::ofstream test_fileB(fnameB);

        // write some not-identical data to the files
        test_fileA << "test file contents";
        test_fileB << "test file contènts";// è instead of e
      }

      // Check that files are NOT equal
      auto cmp_res = io_util::is_files_equal(fnameA, fnameB);
      CHECK(cmp_res.has_value());
      CHECK_FALSE(cmp_res.value());

      // Clean up by removing the files
      std::filesystem::remove(fnameA);
      std::filesystem::remove(fnameB);
    }

    SECTION("Compare two files in two subdirectories with different contents")
    {
      // Create two files in two different directories
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };
      const std::string dirA{ "test_ident_dirA" };
      const std::string dirB{ "test_ident_dirB" };
      const std::string test_file_contents =
        R"([{"id":110465,"quantity":1,"name":"Tranquil Cove","set":"MOM","rarity":"C","foil":true,"goatbots_price":0.004}])";

      // Make the directories
      std::filesystem::create_directory(dirA);
      std::filesystem::create_directory(dirB);

      // Paths to the files
      const std::filesystem::path pathA = dirA + "/" + fnameA;
      const std::filesystem::path pathB = dirB + "/" + fnameB;

      // Save the files
      {
        std::ofstream test_fileA(pathA);
        std::ofstream test_fileB(pathB);

        // write some not-identical data to the files
        test_fileA << test_file_contents;
        test_fileB << test_file_contents + "extra stuff";
      }

      // Check that files are NOT equal
      auto cmp_res = io_util::is_files_equal(pathA, pathB);
      CHECK(cmp_res.has_value());
      CHECK_FALSE(cmp_res.value());

      // Clean up by removing the files and directories
      std::filesystem::remove_all(dirA);
      std::filesystem::remove_all(dirB);
    }

    SECTION("Compare two NOT identical files - one in parent directory, one in current directory")
    {
      // Create two files in two different directories
      const std::string fnameA{ "test_ident_fileA" };
      const std::string fnameB{ "test_ident_fileB" };
      const std::string dirA{ "test_ident_dirA" };

      const std::string contents{ "contents'~~ _[@]... log. test content///DEe`2èøøæ" };

      // Make the directories
      std::filesystem::create_directory(dirA);

      // Paths to the files
      const std::filesystem::path pathA = dirA + "/" + fnameA;
      const std::filesystem::path pathB = fnameB;

      // Save the files
      {
        std::ofstream test_fileA(pathA);
        std::ofstream test_fileB(pathB);

        // write some not-identical data to the files
        test_fileA << contents + '+';
        test_fileB << contents;
      }

      // Check that files are NOT equal
      auto cmp_res = io_util::is_files_equal(pathA, pathB);
      CHECK(cmp_res.has_value());
      CHECK_FALSE(cmp_res.value());

      // Clean up by removing the files and directories
      std::filesystem::remove_all(dirA);
      std::filesystem::remove(fnameB);
    }
  }
}

TEST_CASE("io_util::get_files_with_timestamp")
{
  SECTION("2 files year differs")
  {
    // Create two files with different timestamps
    const std::string newer_timestamp{ "2023-11-14T113236Z" };
    const std::string older_timestamp{ "2022-11-14T113236Z" };
    const std::string newer{ "mtgo-cards_" + newer_timestamp };
    const std::string older{ "mtgo-cards_" + older_timestamp };
    const std::string sub_dir{ "collection-history" };

    std::filesystem::create_directory(sub_dir);

    // Paths to the files
    const std::filesystem::path newer_path = sub_dir + "/" + newer;
    const std::filesystem::path older_path = sub_dir + "/" + older;

    // Save the files
    {
      std::ofstream test_fileA(newer_path);
      std::ofstream test_fileB(older_path);
    }

    // Get the files sorted by timestamp
    auto files = io_util::get_files_with_timestamp(sub_dir);
    CHECK(files.at(0).fpath_ == older_path);
    CHECK(files.at(0).timestamp_ == older_timestamp);
    CHECK(files.at(1).fpath_ == newer_path);
    CHECK(files.at(1).timestamp_ == newer_timestamp);

    // Clean up by removing the files and directory
    std::filesystem::remove_all(sub_dir);
  }

  SECTION("2 files - day and time differs")
  {
    // Create two files with different timestamps
    const std::string newer_timestamp{ "2023-11-17T075106Z" };
    const std::string older_timestamp{ "2023-11-14T113236Z" };
    const std::string newer{ "mtgo-cards_" + newer_timestamp };
    const std::string older{ "mtgo-cards_" + older_timestamp };
    const std::string sub_dir{ "collection-history" };

    std::filesystem::create_directory(sub_dir);

    // Paths to the files
    const std::filesystem::path newer_path = sub_dir + "/" + newer;
    const std::filesystem::path older_path = sub_dir + "/" + older;

    // Save the files
    {
      std::ofstream test_fileA(newer_path);
      std::ofstream test_fileB(older_path);
    }

    // Get the files sorted by timestamp
    auto files = io_util::get_files_with_timestamp(sub_dir);
    CHECK(files.at(0).fpath_ == older_path);
    CHECK(files.at(0).timestamp_ == older_timestamp);
    CHECK(files.at(1).fpath_ == newer_path);
    CHECK(files.at(1).timestamp_ == newer_timestamp);

    // Clean up by removing the files and directory
    std::filesystem::remove_all(sub_dir);
  }

  SECTION("5 files - all different")
  {
    // Create two files with different timestamps
    const std::string oldest_timestamp{ "2022-11-05T075106Z" };
    const std::string second_timestamp{ "2023-01-00T003236Z" };
    const std::string third_timestamp{ "2023-09-14T113236Z" };
    const std::string fourth_timestamp{ "2023-11-14T113236Z" };
    const std::string fifth_timestamp{ "2023-11-14T123236Z" };
    const std::string oldest{ "mtgo-cards_" + oldest_timestamp };
    const std::string second{ "mtgo-cards_" + second_timestamp };
    const std::string third{ "mtgo-cards_" + third_timestamp };
    const std::string fourth{ "mtgo-cards_" + fourth_timestamp };
    const std::string fifth{ "mtgo-cards_" + fifth_timestamp };
    const std::string sub_dir{ "collection-history" };

    std::filesystem::create_directory(sub_dir);

    // Paths to the files
    const std::filesystem::path oldest_path = sub_dir + "/" + oldest;
    const std::filesystem::path second_path = sub_dir + "/" + second;
    const std::filesystem::path third_path = sub_dir + "/" + third;
    const std::filesystem::path fourth_path = sub_dir + "/" + fourth;
    const std::filesystem::path fifth_path = sub_dir + "/" + fifth;

    // Save the files
    {
      std::ofstream file_oldest(oldest_path);
      std::ofstream file_second(second_path);
      std::ofstream file_third(third_path);
      std::ofstream file_fourth(fourth_path);
      std::ofstream file_fifth(fifth_path);
    }

    // Get the files sorted by timestamp
    auto files = io_util::get_files_with_timestamp(sub_dir);

    CHECK(files.at(0).fpath_ == oldest_path);
    CHECK(files.at(0).timestamp_ == oldest_timestamp);

    CHECK(files.at(1).fpath_ == second_path);
    CHECK(files.at(1).timestamp_ == second_timestamp);

    CHECK(files.at(2).fpath_ == third_path);
    CHECK(files.at(2).timestamp_ == third_timestamp);

    CHECK(files.at(3).fpath_ == fourth_path);
    CHECK(files.at(3).timestamp_ == fourth_timestamp);

    CHECK(files.at(4).fpath_ == fifth_path);
    CHECK(files.at(4).timestamp_ == fifth_timestamp);


    // Clean up by removing the files and directory
    std::filesystem::remove_all(sub_dir);
  }
}

TEST_CASE("Compression with io_util::gzip::compress")
{
  SECTION("Compress simple string")
  {
    const std::string test_raw_contents = "test file contents";
    // This compression is counter-productive as the string is so short. Just the headers of Gzip are 20 bytes.
    const std::string compressed = io_util::gzip::compress(test_raw_contents);

    // Check that the compressed string is not empty
    CHECK_FALSE(compressed.empty());

    // Check that the compressed string is not the same as the original string
    CHECK(compressed != test_raw_contents);

    const std::string decompressed = io_util::gzip::decompress(compressed);

    // Check that the decompressed string is the same as the original string
    CHECK(decompressed == test_raw_contents);
  }

  SECTION("Compress a string incrementing from 1 to 33")
  {
    // There's very little repetition so this 57 byte string is still 57 bytes after compression
    const std::string raw_str = "123456789101112131415161718192021222324252627282930313233";

    const std::string compressed = io_util::gzip::compress(raw_str);

    const std::string decompressed = io_util::gzip::decompress(compressed);

    // Check that the decompressed string is the same as the original string
    CHECK(decompressed == raw_str);
    CHECK(decompressed.size() == raw_str.size());
  }

  SECTION("Compress a string of 24 '1's")
  {
    // This is very compressible as there's a lot of repetition
    // The header of Gzip is 20 bytes, and then it takes 3 bytes to encode the length of the compressed string
    // One for the character '1', one for the number of repetitions `18`(hex)
    const std::string raw_str = "111111111111111111111111";

    const std::string compressed = io_util::gzip::compress(raw_str);
    CHECK(compressed.size() == 23);

    const std::string decompressed = io_util::gzip::decompress(compressed);

    // Check that the decompressed string is the same as the original string
    CHECK(decompressed == raw_str);
  }
}

// NOLINTEND