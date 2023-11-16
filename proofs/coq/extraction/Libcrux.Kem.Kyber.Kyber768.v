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

Definition v_RANK_768_ : uint_size :=
  (@repr WORDSIZE32 3).

Definition v_CPA_PKE_SECRET_KEY_SIZE_768_ : uint_size :=
  ((v_RANK_768_.*v_COEFFICIENTS_IN_RING_ELEMENT).*v_BITS_PER_COEFFICIENT)./(@repr WORDSIZE32 8).

Definition v_RANKED_BYTES_PER_RING_ELEMENT_768_ : uint_size :=
  (v_RANK_768_.*v_BITS_PER_RING_ELEMENT)./(@repr WORDSIZE32 8).

Definition v_T_AS_NTT_ENCODED_SIZE_768_ : uint_size :=
  ((v_RANK_768_.*v_COEFFICIENTS_IN_RING_ELEMENT).*v_BITS_PER_COEFFICIENT)./(@repr WORDSIZE32 8).

Definition v_CPA_PKE_PUBLIC_KEY_SIZE_768_ : uint_size :=
  v_T_AS_NTT_ENCODED_SIZE_768_.+(@repr WORDSIZE32 32).

Definition v_SECRET_KEY_SIZE_768_ : uint_size :=
  ((v_CPA_PKE_SECRET_KEY_SIZE_768_.+v_CPA_PKE_PUBLIC_KEY_SIZE_768_).+v_H_DIGEST_SIZE).+v_SHARED_SECRET_SIZE.

Definition v_VECTOR_U_COMPRESSION_FACTOR_768_ : uint_size :=
  (@repr WORDSIZE32 10).

Definition v_C1_BLOCK_SIZE_768_ : uint_size :=
  (v_COEFFICIENTS_IN_RING_ELEMENT.*v_VECTOR_U_COMPRESSION_FACTOR_768_)./(@repr WORDSIZE32 8).

Definition v_C1_SIZE_768_ : uint_size :=
  v_C1_BLOCK_SIZE_768_.*v_RANK_768_.

Definition v_VECTOR_V_COMPRESSION_FACTOR_768_ : uint_size :=
  (@repr WORDSIZE32 4).

Definition v_C2_SIZE_768_ : uint_size :=
  (v_COEFFICIENTS_IN_RING_ELEMENT.*v_VECTOR_V_COMPRESSION_FACTOR_768_)./(@repr WORDSIZE32 8).

Definition v_CPA_PKE_CIPHERTEXT_SIZE_768_ : uint_size :=
  v_C1_SIZE_768_.+v_C2_SIZE_768_.

Definition v_IMPLICIT_REJECTION_HASH_INPUT_SIZE : uint_size :=
  v_SHARED_SECRET_SIZE.+v_CPA_PKE_CIPHERTEXT_SIZE_768_.

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)

(*item error backend*)
