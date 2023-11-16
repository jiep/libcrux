(* File automatically generated by Hacspec *)
From Hacspec Require Import Hacspec_Lib MachineIntegers.
From Coq Require Import ZArith.
Import List.ListNotations.
Open Scope Z_scope.
Open Scope bool_scope.

Require Import Super.

Require Import Super.

Require Import Super.

Definition v_ETA1 : uint_size :=
  (@repr WORDSIZE32 2).

Definition v_ETA1_RANDOMNESS_SIZE : uint_size :=
  v_ETA1.*(@repr WORDSIZE32 64).

Definition v_ETA2 : uint_size :=
  (@repr WORDSIZE32 2).

Definition v_ETA2_RANDOMNESS_SIZE : uint_size :=
  v_ETA2.*(@repr WORDSIZE32 64).

Definition v_RANK_1024_ : uint_size :=
  (@repr WORDSIZE32 4).

Definition v_CPA_PKE_SECRET_KEY_SIZE_1024_ : uint_size :=
  ((v_RANK_1024_.*v_COEFFICIENTS_IN_RING_ELEMENT).*v_BITS_PER_COEFFICIENT)./(@repr WORDSIZE32 8).

Definition v_RANKED_BYTES_PER_RING_ELEMENT_1024_ : uint_size :=
  (v_RANK_1024_.*v_BITS_PER_RING_ELEMENT)./(@repr WORDSIZE32 8).

Definition v_T_AS_NTT_ENCODED_SIZE_1024_ : uint_size :=
  ((v_RANK_1024_.*v_COEFFICIENTS_IN_RING_ELEMENT).*v_BITS_PER_COEFFICIENT)./(@repr WORDSIZE32 8).

Definition v_CPA_PKE_PUBLIC_KEY_SIZE_1024_ : uint_size :=
  v_T_AS_NTT_ENCODED_SIZE_1024_.+(@repr WORDSIZE32 32).

Definition v_SECRET_KEY_SIZE_1024_ : uint_size :=
  ((v_CPA_PKE_SECRET_KEY_SIZE_1024_.+v_CPA_PKE_PUBLIC_KEY_SIZE_1024_).+v_H_DIGEST_SIZE).+v_SHARED_SECRET_SIZE.

Definition v_VECTOR_U_COMPRESSION_FACTOR_1024_ : uint_size :=
  (@repr WORDSIZE32 11).

Definition v_C1_BLOCK_SIZE_1024_ : uint_size :=
  (v_COEFFICIENTS_IN_RING_ELEMENT.*v_VECTOR_U_COMPRESSION_FACTOR_1024_)./(@repr WORDSIZE32 8).

Definition v_C1_SIZE_1024_ : uint_size :=
  v_C1_BLOCK_SIZE_1024_.*v_RANK_1024_.

Definition v_VECTOR_V_COMPRESSION_FACTOR_1024_ : uint_size :=
  (@repr WORDSIZE32 5).

Definition v_C2_SIZE_1024_ : uint_size :=
  (v_COEFFICIENTS_IN_RING_ELEMENT.*v_VECTOR_V_COMPRESSION_FACTOR_1024_)./(@repr WORDSIZE32 8).

Definition v_CPA_PKE_CIPHERTEXT_SIZE_1024_ : uint_size :=
  v_C1_SIZE_1024_.+v_C2_SIZE_1024_.

Definition v_IMPLICIT_REJECTION_HASH_INPUT_SIZE : uint_size :=
  v_SHARED_SECRET_SIZE.+v_CPA_PKE_CIPHERTEXT_SIZE_1024_.

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)
