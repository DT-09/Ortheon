From Coq Require Import Relations Relation_Operators.

Section RelationAlgebra.
  Context {A : Type}.

  Definition rel := A -> A -> Prop.

  Definition inverse (R : rel) : rel :=
    fun x y => R y x.

  Definition compose (R S : rel) : rel :=
    fun x z => exists y, R x y /\ S y z.

  Definition identity : rel := fun x y => x = y.

  Definition reflexive (R : rel) : Prop :=
    forall x, R x x.

  Definition symmetric (R : rel) : Prop :=
    forall x y, R x y -> R y x.

  Definition transitive (R : rel) : Prop :=
    forall x y z, R x y -> R y z -> R x z.

  Lemma inverse_involutive : forall R x y,
      inverse (inverse R) x y <-> R x y.
  Proof. firstorder. Qed.

  Lemma compose_associative : forall R S T x z,
      compose (compose R S) T x z <->
      compose R (compose S T) x z.
  Proof. firstorder. Qed.

  Lemma compose_identity_left : forall R x y,
      compose identity R x y <-> R x y.
  Proof.
    intros R x y. unfold compose, identity. split.
    - intros [w [Hxy Hwr]]. subst. exact Hwr.
    - intro H. exists x. split; assumption.
  Qed.

  Lemma compose_identity_right : forall R x y,
      compose R identity x y <-> R x y.
  Proof.
    intros R x y. unfold compose, identity. split.
    - intros [w [Hxw Hwy]]. subst. exact Hxw.
    - intro H. exists y. split; assumption.
  Qed.

  Lemma inverse_of_inverse_is_original : forall R,
      forall x y, inverse (inverse R) x y <-> R x y.
  Proof. firstorder. Qed.

  Definition rtc (R : rel) : rel :=
    clos_refl_trans A R.

  Lemma rtc_reflexive : forall R, reflexive (rtc R).
  Proof. intros R x. apply rt_refl. Qed.

  Lemma rtc_contains_relation : forall R x y, R x y -> rtc R x y.
  Proof. intros R x y H. apply rt_step. exact H. Qed.

  Lemma rtc_transitive : forall R, transitive (rtc R).
  Proof.
    intros R x y z Hxy Hyz.
    eapply rt_trans; eauto.
  Qed.
End RelationAlgebra.
