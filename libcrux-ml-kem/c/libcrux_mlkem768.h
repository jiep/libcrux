/* 
  This file was generated by KaRaMeL <https://github.com/FStarLang/karamel>
  KaRaMeL invocation: /Users/jonathan/Code/eurydice/eurydice --config ../c.yaml ../../libcrux_ml_kem.llbc ../../libcrux_sha3.llbc
  F* version: 58c915a8
  KaRaMeL version: 38d348ce
 */

#ifndef __libcrux_mlkem768_H
#define __libcrux_mlkem768_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_sha3.h"
#include "libcrux_polynomial.h"
#include "libcrux_platform.h"
#include "libcrux_core.h"
#include "eurydice_glue.h"

#define LIBCRUX_ML_KEM_MLKEM768_VECTOR_U_COMPRESSION_FACTOR_768 ((size_t)10U)

#define LIBCRUX_ML_KEM_MLKEM768_C1_BLOCK_SIZE_768 (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * LIBCRUX_ML_KEM_MLKEM768_VECTOR_U_COMPRESSION_FACTOR_768 / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_RANK_768 ((size_t)3U)

#define LIBCRUX_ML_KEM_MLKEM768_C1_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_C1_BLOCK_SIZE_768 * LIBCRUX_ML_KEM_MLKEM768_RANK_768)

#define LIBCRUX_ML_KEM_MLKEM768_VECTOR_V_COMPRESSION_FACTOR_768 ((size_t)4U)

#define LIBCRUX_ML_KEM_MLKEM768_C2_SIZE_768 (LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * LIBCRUX_ML_KEM_MLKEM768_VECTOR_V_COMPRESSION_FACTOR_768 / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_C1_SIZE_768 + LIBCRUX_ML_KEM_MLKEM768_C2_SIZE_768)

#define LIBCRUX_ML_KEM_MLKEM768_T_AS_NTT_ENCODED_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_RANK_768 * LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_T_AS_NTT_ENCODED_SIZE_768 + (size_t)32U)

#define LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_SECRET_KEY_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_RANK_768 * LIBCRUX_ML_KEM_CONSTANTS_COEFFICIENTS_IN_RING_ELEMENT * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_COEFFICIENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA1 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA1_RANDOMNESS_SIZE (LIBCRUX_ML_KEM_MLKEM768_ETA1 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA2 ((size_t)2U)

#define LIBCRUX_ML_KEM_MLKEM768_ETA2_RANDOMNESS_SIZE (LIBCRUX_ML_KEM_MLKEM768_ETA2 * (size_t)64U)

#define LIBCRUX_ML_KEM_MLKEM768_IMPLICIT_REJECTION_HASH_INPUT_SIZE (LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE + LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_CIPHERTEXT_SIZE_768)

#define LIBCRUX_ML_KEM_MLKEM768_RANKED_BYTES_PER_RING_ELEMENT_768 (LIBCRUX_ML_KEM_MLKEM768_RANK_768 * LIBCRUX_ML_KEM_CONSTANTS_BITS_PER_RING_ELEMENT / (size_t)8U)

#define LIBCRUX_ML_KEM_MLKEM768_SECRET_KEY_SIZE_768 (LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_SECRET_KEY_SIZE_768 + LIBCRUX_ML_KEM_MLKEM768_CPA_PKE_PUBLIC_KEY_SIZE_768 + LIBCRUX_ML_KEM_CONSTANTS_H_DIGEST_SIZE + LIBCRUX_ML_KEM_CONSTANTS_SHARED_SECRET_SIZE)

void
libcrux_ml_kem_mlkem768_decapsulate(
  libcrux_ml_kem_types_MlKemPrivateKey____2400size_t *private_key,
  libcrux_ml_kem_types_MlKemCiphertext____1088size_t *ciphertext,
  uint8_t ret[32U]
);

K___libcrux_ml_kem_types_MlKemCiphertext___1088size_t___uint8_t_32size_t_
libcrux_ml_kem_mlkem768_encapsulate(
  libcrux_ml_kem_types_MlKemPublicKey____1184size_t *public_key,
  uint8_t randomness[32U]
);

libcrux_ml_kem_types_MlKemKeyPair____2400size_t__1184size_t
libcrux_ml_kem_mlkem768_generate_key_pair(uint8_t randomness[64U]);

core_option_Option__libcrux_ml_kem_types_MlKemPublicKey___1184size_t__
libcrux_ml_kem_mlkem768_validate_public_key(
  libcrux_ml_kem_types_MlKemPublicKey____1184size_t public_key
);

#if defined(__cplusplus)
}
#endif

#define __libcrux_mlkem768_H_DEFINED
#endif
