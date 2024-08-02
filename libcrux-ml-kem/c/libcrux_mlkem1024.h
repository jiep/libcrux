/*
 * SPDX-FileCopyrightText: 2024 Cryspen Sarl <info@cryspen.com>
 *
 * SPDX-License-Identifier: MIT or Apache-2.0
 *
 * This code was generated with the following revisions:
 * Charon: 45b95e0f63cb830202c0b3ca00a341a3451a02ba
 * Eurydice: 8f3c82290a95695e4f6bbaebc8317bfbe03233be
 * Karamel: d43a65c629e989afd6b21fa4486feda78a190a47
 * F*: b2931dfbe46e839cd757220c63d48c71335bb1ae
 * Libcrux: a23ad09369f2361317bfd6fd43267bd8521048eb
 */

#ifndef __libcrux_mlkem1024_H
#define __libcrux_mlkem1024_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "eurydice_glue.h"
#include "libcrux_core.h"

#define LIBCRUX_ML_KEM_MLKEM1024_VECTOR_U_COMPRESSION_FACTOR_1024 ((size_t)11U)

#define LIBCRUX_ML_KEM_MLKEM1024_C1_BLOCK_SIZE_1024        \
  (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_MLKEM1024_VECTOR_U_COMPRESSION_FACTOR_1024 / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM1024_RANK_1024 ((size_t)4U)

#define LIBCRUX_ML_KEM_MLKEM1024_C1_SIZE_1024    \
  (LIBCRUX_ML_KEM_MLKEM1024_C1_BLOCK_SIZE_1024 * \
   LIBCRUX_ML_KEM_MLKEM1024_RANK_1024)

#define LIBCRUX_ML_KEM_MLKEM1024_VECTOR_V_COMPRESSION_FACTOR_1024 ((size_t)5U)

#define LIBCRUX_ML_KEM_MLKEM1024_C2_SIZE_1024              \
  (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * \
   LIBCRUX_ML_KEM_MLKEM1024_VECTOR_V_COMPRESSION_FACTOR_1024 / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE_1024 \
  (LIBCRUX_ML_KEM_MLKEM1024_C1_SIZE_1024 +                    \
   LIBCRUX_ML_KEM_MLKEM1024_C2_SIZE_1024)

#define LIBCRUX_ML_KEM_MLKEM1024_T_AS_NTT_ENCODED_SIZE_1024 \
  (LIBCRUX_ML_KEM_MLKEM1024_RANK_1024 *                     \
   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *  \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_PUBLIC_KEY_SIZE_1024 \
  (LIBCRUX_ML_KEM_MLKEM1024_T_AS_NTT_ENCODED_SIZE_1024 + (size_t)32U)

#define LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_SECRET_KEY_SIZE_1024 \
  (LIBCRUX_ML_KEM_MLKEM1024_RANK_1024 *                       \
   LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT *    \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM1024_ETA1 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM1024_ETA1_RANDOMNESS_SIZE \
  (LIBCRUX_ML_KEM_MLKEM1024_ETA1 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM1024_ETA2 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM1024_ETA2_RANDOMNESS_SIZE \
  (LIBCRUX_ML_KEM_MLKEM1024_ETA2 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM1024_IMPLICIT_REJECTION_HASH_INPUT_SIZE \
  (LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE +                    \
   LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_CIPHERTEXT_SIZE_1024)

typedef libcrux_ml_kem_types_MlKemPrivateKey_95
    libcrux_ml_kem_mlkem1024_MlKem1024PrivateKey;

typedef libcrux_ml_kem_types_MlKemPublicKey_1f
    libcrux_ml_kem_mlkem1024_MlKem1024PublicKey;

#define LIBCRUX_ML_KEM_MLKEM1024_RANKED_BYTES_PER_RING_ELEMENT_1024 \
  (LIBCRUX_ML_KEM_MLKEM1024_RANK_1024 *                             \
   LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM1024_SECRET_KEY_SIZE_1024      \
  (LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_SECRET_KEY_SIZE_1024 + \
   LIBCRUX_ML_KEM_MLKEM1024_CPA_PKE_PUBLIC_KEY_SIZE_1024 + \
   LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE +                \
   LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE)

#if defined(__cplusplus)
}
#endif

#define __libcrux_mlkem1024_H_DEFINED
#endif
