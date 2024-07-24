module Libcrux_ml_kem.Vector.Portable.Vector_type
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open Core
open FStar.Mul

type t_MaxPortableVector (v_MIN: i16) (v_MAX: i16) = {
  f_elements:t_Array (Hax_bounded_integers.t_BoundedI16 v_MIN v_MAX) (sz 16)
}

type t_PortableVector = { f_elements:t_Array i16 (sz 16) }

val from_i16_array (array: t_Slice i16)
    : Prims.Pure t_PortableVector Prims.l_True (fun _ -> Prims.l_True)

val max_zero: v_MIN: i16 -> v_MAX: i16 -> Prims.unit
  -> Prims.Pure (t_MaxPortableVector v_MIN v_MAX) Prims.l_True (fun _ -> Prims.l_True)

val to_i16_array (x: t_PortableVector)
    : Prims.Pure (t_Array i16 (sz 16)) Prims.l_True (fun _ -> Prims.l_True)

val zero: Prims.unit -> Prims.Pure t_PortableVector Prims.l_True (fun _ -> Prims.l_True)
