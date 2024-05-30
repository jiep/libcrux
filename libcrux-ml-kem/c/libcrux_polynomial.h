/* 
  This file was generated by KaRaMeL <https://github.com/FStarLang/karamel>
  KaRaMeL invocation: /Users/jonathan/Code/eurydice/eurydice --config ../c.yaml ../../libcrux_ml_kem.llbc ../../libcrux_sha3.llbc
  F* version: 58c915a8
  KaRaMeL version: 38d348ce
 */

#ifndef __libcrux_polynomial_H
#define __libcrux_polynomial_H

#if defined(__cplusplus)
extern "C" {
#endif

#include "libcrux_core.h"
#include "eurydice_glue.h"

typedef struct libcrux_ml_kem_vector_PortableVector_s { int16_t elements[16U]; }
libcrux_ml_kem_vector_PortableVector;

typedef struct
libcrux_ml_kem_polynomial_PolynomialRingElement__libcrux_ml_kem_vector_PortableVector_s
{ libcrux_ml_kem_vector_PortableVector coefficients[16U]; }
libcrux_ml_kem_polynomial_PolynomialRingElement__libcrux_ml_kem_vector_PortableVector;

#if defined(__cplusplus)
}
#endif

#define __libcrux_polynomial_H_DEFINED
#endif
