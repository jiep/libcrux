/*
 * SPDX-FileCopyrightText: 2024 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

/*
  This file was generated by KaRaMeL <https://github.com/FStarLang/karamel>
  KaRaMeL invocation: /Users/franziskus/repos/eurydice//eurydice --config
  ../c2.yml ../../libcrux_ml_kem.llbc F* version: a32b316e KaRaMeL version:
  020de30f
 */

#ifndef __libcrux_intrinsics_arm64_H
#define __libcrux_intrinsics_arm64_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <arm_neon.h>

#include "eurydice_glue.h"

// NEON Vector Types
typedef int16x4_t core_core_arch_arm_shared_neon_int16x4_t;
typedef uint16x4_t core_core_arch_arm_shared_neon_uint16x4_t;
typedef int8x16_t core_core_arch_arm_shared_neon_int8x16_t;
typedef uint8x16_t core_core_arch_arm_shared_neon_uint8x16_t;
typedef int16x8_t core_core_arch_arm_shared_neon_int16x8_t;
typedef uint16x8_t core_core_arch_arm_shared_neon_uint16x8_t;
typedef int32x4_t core_core_arch_arm_shared_neon_int32x4_t;
typedef uint32x4_t core_core_arch_arm_shared_neon_uint32x4_t;
typedef int64x2_t core_core_arch_arm_shared_neon_int64x2_t;
typedef uint64x2_t core_core_arch_arm_shared_neon_uint64x2_t;

// Casting Between Vector Types
static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vreinterpretq_s16_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a) {
  return vreinterpretq_s16_u16(a);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vreinterpretq_u16_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vreinterpretq_u16_s16(a);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vreinterpretq_u32_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vreinterpretq_u32_s16(a);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vreinterpretq_s32_u32(
    core_core_arch_arm_shared_neon_uint32x4_t a) {
  return vreinterpretq_s32_u32(a);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vreinterpretq_u32_s32(
    core_core_arch_arm_shared_neon_int32x4_t a) {
  return vreinterpretq_u32_s32(a);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vreinterpretq_s16_u32(
    core_core_arch_arm_shared_neon_uint32x4_t a) {
  return vreinterpretq_s16_u32(a);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vreinterpretq_s32_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vreinterpretq_s32_s16(a);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vreinterpretq_s16_s32(
    core_core_arch_arm_shared_neon_int32x4_t a) {
  return vreinterpretq_s16_s32(a);
}

static inline core_core_arch_arm_shared_neon_int64x2_t
libcrux_intrinsics_arm64__vreinterpretq_s64_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vreinterpretq_s64_s16(a);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vreinterpretq_s16_s64(
    core_core_arch_arm_shared_neon_int64x2_t a) {
  return vreinterpretq_s16_s64(a);
}

static inline core_core_arch_arm_shared_neon_uint8x16_t
libcrux_intrinsics_arm64__vreinterpretq_u8_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vreinterpretq_u8_s16(a);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vreinterpretq_s16_u8(
    core_core_arch_arm_shared_neon_uint8x16_t a) {
  return vreinterpretq_s16_u8(a);
}

static inline core_core_arch_arm_shared_neon_int64x2_t
libcrux_intrinsics_arm64__vreinterpretq_s64_s32(
    core_core_arch_arm_shared_neon_int32x4_t a) {
  return vreinterpretq_s64_s32(a);
}

static inline core_core_arch_arm_shared_neon_uint8x16_t
libcrux_intrinsics_arm64__vreinterpretq_u8_s64(
    core_core_arch_arm_shared_neon_int64x2_t a) {
  return vreinterpretq_u8_s64(a);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vreinterpretq_u16_u8(
    core_core_arch_arm_shared_neon_uint8x16_t a) {
  return vreinterpretq_u16_u8(a);
}

// Initialize, Load, Store

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vdupq_n_s16(int16_t c) {
  return vdupq_n_s16(c);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vdupq_n_u32(uint32_t a) {
  return vdupq_n_u32(a);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vdupq_n_u16(uint16_t value) {
  return vdupq_n_u16(value);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vld1q_s16(Eurydice_slice slice) {
  return vld1q_s16((int16_t*)slice.ptr);
}

static inline core_core_arch_arm_shared_neon_uint8x16_t
libcrux_intrinsics_arm64__vld1q_u8(Eurydice_slice slice) {
  return vld1q_u8((uint8_t*)slice.ptr);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vld1q_u16(Eurydice_slice slice) {
  return vld1q_u16((uint16_t*)slice.ptr);
}

static inline void libcrux_intrinsics_arm64__vst1q_s16(
    Eurydice_slice out, core_core_arch_arm_shared_neon_int16x8_t a) {
  vst1q_s16((int16_t*)out.ptr, a);
}

static inline void libcrux_intrinsics_arm64__vst1q_u8(
    Eurydice_slice out, core_core_arch_arm_shared_neon_uint8x16_t a) {
  vst1q_u8((uint8_t*)out.ptr, a);
}

// Arithmetic: Add, Sub

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vaddq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vaddq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vaddq_u32(
    core_core_arch_arm_shared_neon_uint32x4_t a,
    core_core_arch_arm_shared_neon_uint32x4_t b) {
  return vaddq_u32(a, b);
}

static inline int16_t libcrux_intrinsics_arm64__vaddvq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vaddvq_s16(a);
}

static inline uint16_t libcrux_intrinsics_arm64__vaddv_u16(
    core_core_arch_arm_shared_neon_uint16x4_t a) {
  return vaddv_u16(a);
}

static inline uint16_t libcrux_intrinsics_arm64__vaddvq_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a) {
  return vaddvq_u16(a);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vsubq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vsubq_s16(a, b);
}

// Arithmetic: Mul, Mul-High, Mul-Add

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vmulq_n_s16(
    core_core_arch_arm_shared_neon_int16x8_t a, int16_t c) {
  return vmulq_n_s16(a, c);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vqdmulhq_n_s16(
    core_core_arch_arm_shared_neon_int16x8_t a, int16_t c) {
  return vqdmulhq_n_s16(a, c);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vmulq_n_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a, uint16_t c) {
  return vmulq_n_u16(a, c);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vqdmulhq_n_s32(
    core_core_arch_arm_shared_neon_int32x4_t a, int32_t c) {
  return vqdmulhq_n_s32(a, c);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vmulq_n_u32(
    core_core_arch_arm_shared_neon_uint32x4_t a, uint32_t c) {
  return vmulq_n_u32(a, c);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vmulq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vmulq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vqdmulhq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vqdmulhq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vmull_s16(
    core_core_arch_arm_shared_neon_int16x4_t a,
    core_core_arch_arm_shared_neon_int16x4_t b) {
  return vmull_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vmull_high_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vmull_high_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vmlal_s16(
    core_core_arch_arm_shared_neon_int32x4_t a,
    core_core_arch_arm_shared_neon_int16x4_t b,
    core_core_arch_arm_shared_neon_int16x4_t c) {
  return vmlal_s16(a, b, c);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vmlal_high_s16(
    core_core_arch_arm_shared_neon_int32x4_t a,
    core_core_arch_arm_shared_neon_int16x8_t b,
    core_core_arch_arm_shared_neon_int16x8_t c) {
  return vmlal_high_s16(a, b, c);
}

// Comparisons

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vcgeq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vcgeq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vcleq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vcleq_s16(a, b);
}

// Bitwise Operations
static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vandq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vandq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__veorq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return veorq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_uint32x4_t
libcrux_intrinsics_arm64__vandq_u32(
    core_core_arch_arm_shared_neon_uint32x4_t a,
    core_core_arch_arm_shared_neon_uint32x4_t b) {
  return vandq_u32(a, b);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vandq_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a,
    core_core_arch_arm_shared_neon_uint16x8_t b) {
  return vandq_u16(a, b);
}

// Shift Operations

#define libcrux_intrinsics_arm64__vshrq_n_s16(SHIFT_BY, a, _ret_t) \
  (vshrq_n_s16(a, SHIFT_BY))

#define libcrux_intrinsics_arm64__vshrq_n_u16(SHIFT_BY, a, _ret_t) \
  (vshrq_n_u16(a, SHIFT_BY))

#define libcrux_intrinsics_arm64__vshrq_n_u32(N, a, _ret_t) (vshrq_n_u32(a, N))

#define libcrux_intrinsics_arm64__vshlq_n_u32(N, a, _ret_t) (vshlq_n_u32(a, N))

#define libcrux_intrinsics_arm64__vshlq_n_s16(SHIFT_BY, a, _ret_t) \
  (vshlq_n_s16(a, SHIFT_BY))

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vshlq_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vshlq_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_uint16x8_t
libcrux_intrinsics_arm64__vshlq_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vshlq_u16(a, b);
}

#define libcrux_intrinsics_arm64__vsliq_n_s32(N, a, b, _ret_t) \
  (vsliq_n_s32(a, b, N))

#define libcrux_intrinsics_arm64__vsliq_n_s64(N, a, b, _ret_t) \
  (vsliq_n_s64(a, b, N))

// Transpose and Vector Manipulations

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vtrn1q_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vtrn1q_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int16x8_t
libcrux_intrinsics_arm64__vtrn2q_s16(
    core_core_arch_arm_shared_neon_int16x8_t a,
    core_core_arch_arm_shared_neon_int16x8_t b) {
  return vtrn2q_s16(a, b);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vtrn1q_s32(
    core_core_arch_arm_shared_neon_int32x4_t a,
    core_core_arch_arm_shared_neon_int32x4_t b) {
  return vtrn1q_s32(a, b);
}

static inline core_core_arch_arm_shared_neon_int32x4_t
libcrux_intrinsics_arm64__vtrn2q_s32(
    core_core_arch_arm_shared_neon_int32x4_t a,
    core_core_arch_arm_shared_neon_int32x4_t b) {
  return vtrn2q_s32(a, b);
}

static inline core_core_arch_arm_shared_neon_int64x2_t
libcrux_intrinsics_arm64__vtrn1q_s64(
    core_core_arch_arm_shared_neon_int64x2_t a,
    core_core_arch_arm_shared_neon_int64x2_t b) {
  return vtrn1q_s64(a, b);
}

static inline core_core_arch_arm_shared_neon_int64x2_t
libcrux_intrinsics_arm64__vtrn2q_s64(
    core_core_arch_arm_shared_neon_int64x2_t a,
    core_core_arch_arm_shared_neon_int64x2_t b) {
  return vtrn2q_s64(a, b);
}

static inline core_core_arch_arm_shared_neon_int16x4_t
libcrux_intrinsics_arm64__vget_low_s16(
    core_core_arch_arm_shared_neon_int16x8_t a) {
  return vget_low_s16(a);
}

static inline core_core_arch_arm_shared_neon_uint16x4_t
libcrux_intrinsics_arm64__vget_low_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a) {
  return vget_low_u16(a);
}

static inline core_core_arch_arm_shared_neon_uint16x4_t
libcrux_intrinsics_arm64__vget_high_u16(
    core_core_arch_arm_shared_neon_uint16x8_t a) {
  return vget_high_u16(a);
}

static inline core_core_arch_arm_shared_neon_uint8x16_t
libcrux_intrinsics_arm64__vqtbl1q_u8(
    core_core_arch_arm_shared_neon_uint8x16_t a,
    core_core_arch_arm_shared_neon_uint8x16_t b) {
  return vqtbl1q_u8(a, b);
}

#define libcrux_intrinsics_arm64__vshlq_n_u64(SHIFT_BY, x, _ret_t) \
  (vshlq_n_u64(x, SHIFT_BY))

#define libcrux_intrinsics_arm64__vshrq_n_u64(SHIFT_BY, x, _ret_t) \
  (vshrq_n_u64(x, SHIFT_BY))

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__veorq_u64(
    core_core_arch_arm_shared_neon_uint64x2_t x0,
    core_core_arch_arm_shared_neon_uint64x2_t x1) {
  return veorq_u64(x0, x1);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vbicq_u64(
    core_core_arch_arm_shared_neon_uint64x2_t x0,
    core_core_arch_arm_shared_neon_uint64x2_t x1) {
  return vbicq_u64(x0, x1);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vdupq_n_u64(uint64_t x0) {
  return vdupq_n_u64(x0);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vld1q_bytes_u64(Eurydice_slice x0) {
  return vld1q_u64((uint64_t*)x0.ptr);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vtrn1q_u64(
    core_core_arch_arm_shared_neon_uint64x2_t x0,
    core_core_arch_arm_shared_neon_uint64x2_t x1) {
  return vtrn1q_u64(x0, x1);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vtrn2q_u64(
    core_core_arch_arm_shared_neon_uint64x2_t x0,
    core_core_arch_arm_shared_neon_uint64x2_t x1) {
  return vtrn2q_u64(x0, x1);
}

static inline core_core_arch_arm_shared_neon_uint64x2_t
libcrux_intrinsics_arm64__vld1q_u64(Eurydice_slice x0) {
  return vld1q_u64((uint64_t*)x0.ptr);
}

static inline void libcrux_intrinsics_arm64__vst1q_bytes_u64(
    Eurydice_slice x0, core_core_arch_arm_shared_neon_uint64x2_t x1) {
  vst1q_u64((uint64_t*)x0.ptr, x1);
}

#if defined(__cplusplus)
}
#endif

#define __libcrux_intrinsics_arm64_H_DEFINED
#endif
