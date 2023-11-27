#pragma once

#include <cstring>
#include <stdexcept>
#include <string>
#include <vector>
// NOLINTBEGIN
#if defined(_MSC_VER)
// TODO: compression for windows
#else
#include "zlib.h"
#endif
// NOLINTEND

namespace io_util::compression {

[[nodiscard]] inline auto compress(const std::string &data) -> std::string
#if defined(_MSC_VER)
{
  throw std::runtime_error("Compression not supported on Windows.");
}
#else
{
  z_stream zs;
  memset(&zs, 0, sizeof(zs));

  /* Initialize compression system -
   * https://refspecs.linuxbase.org/LSB_3.0.0/LSB-Core-generic/LSB-Core-generic/zlib-deflateinit2.html */
  if (deflateInit2(&zs, Z_BEST_COMPRESSION, Z_DEFLATED, 15 | 16, 8, Z_DEFAULT_STRATEGY) != Z_OK) {
    throw std::runtime_error("deflateInit2 failed - failure to initialize compression system!");
  }

  zs.next_in = reinterpret_cast<Bytef *>(const_cast<char *>(data.data()));
  zs.avail_in = static_cast<uInt>(data.size());

  int ret;
  char buffer[32768];// 32K buffer
  std::string compressed;

  do {
    zs.next_out = reinterpret_cast<Bytef *>(buffer);
    zs.avail_out = sizeof(buffer);

    // Compress data - https://refspecs.linuxbase.org/LSB_3.0.0/LSB-Core-generic/LSB-Core-generic/zlib-deflate-1.html
    // Setting deflate()'s flush parameter to Z_FINISH tells it to stop when it has finished compressing all the data
    // if there's more data to be compressed, it will return with a status of Z_OK, then next_out and avail_out should
    // be updated and deflate() should be called again (with flush set to Z_FINISH again) as it happens in this loop.
    // When deflate() returns with a status of Z_STREAM_END, all the data has been compressed and the loop can be
    // exited.
    ret = deflate(&zs, Z_FINISH);

    // Append to output string, check for size mismatch to avoid appending if deflate() didn't write anything.
    if (compressed.size() < zs.total_out) { compressed.append(buffer, zs.total_out - compressed.size()); }
  } while (ret == Z_OK);

  // Free compression stream state -
  // https://refspecs.linuxbase.org/LSB_3.0.0/LSB-Core-generic/LSB-Core-generic/zlib-deflateend-1.html
  deflateEnd(&zs);

  // Check for errors, Z_STREAM_END is expected here, anything else is an error.
  if (ret != Z_STREAM_END) { throw std::runtime_error("Error while compressing: " + std::to_string(ret)); }

  return compressed;
}
#endif

[[nodiscard]] inline auto decompress(const std::string &data) -> std::string
#if defined(_MSC_VER)
{
  throw std::runtime_error("Decompression not supported on Windows.");
}
#else
{
  z_stream zs;
  memset(&zs, 0, sizeof(zs));

  if (inflateInit2(&zs, 16 + MAX_WBITS) != Z_OK) {
    throw std::runtime_error("inflateInit failed while decompressing.");
  }

  zs.next_in = reinterpret_cast<Bytef *>(const_cast<char *>(data.data()));
  zs.avail_in = static_cast<uInt>(data.size());

  int ret;
  char buffer[32768];// 32K buffer
  std::string decompressed;

  do {
    zs.next_out = reinterpret_cast<Bytef *>(buffer);
    zs.avail_out = sizeof(buffer);

    ret = inflate(&zs, 0);

    if (decompressed.size() < zs.total_out) { decompressed.append(buffer, zs.total_out - decompressed.size()); }
  } while (ret == Z_OK);

  inflateEnd(&zs);

  if (ret != Z_STREAM_END) { throw std::runtime_error("Error while decompressing: " + std::to_string(ret)); }

  return decompressed;
}
#endif

}// namespace io_util::compression