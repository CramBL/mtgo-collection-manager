#pragma once

#include <cstring>
#include <stdexcept>
#include <string>
#include <vector>

#include "zlib.h"

namespace io_util::gzip {


[[nodiscard]] inline auto compress(const std::string &data) -> std::string
{
  z_stream zs;
  memset(&zs, 0, sizeof(zs));

  if (deflateInit2(&zs, Z_BEST_COMPRESSION, Z_DEFLATED, 15 | 16, 8, Z_DEFAULT_STRATEGY) != Z_OK) {
    throw std::runtime_error("deflateInit failed while compressing.");
  }

  zs.next_in = reinterpret_cast<Bytef *>(const_cast<char *>(data.data()));
  zs.avail_in = static_cast<uInt>(data.size());

  int ret;
  char buffer[32768];// 32K buffer
  std::string compressed;

  do {
    zs.next_out = reinterpret_cast<Bytef *>(buffer);
    zs.avail_out = sizeof(buffer);

    ret = deflate(&zs, Z_FINISH);

    if (compressed.size() < zs.total_out) { compressed.append(buffer, zs.total_out - compressed.size()); }
  } while (ret == Z_OK);

  deflateEnd(&zs);

  if (ret != Z_STREAM_END) { throw std::runtime_error("Error while compressing: " + std::to_string(ret)); }

  return compressed;
}

[[nodiscard]] inline auto decompress(const std::string &data) -> std::string
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

}// namespace io_util::gzip