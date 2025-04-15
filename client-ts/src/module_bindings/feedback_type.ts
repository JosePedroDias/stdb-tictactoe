// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
import {
  AlgebraicType,
  AlgebraicValue,
  BinaryReader,
  BinaryWriter,
  CallReducerFlags,
  ConnectionId,
  DbConnectionBuilder,
  DbConnectionImpl,
  DbContext,
  ErrorContextInterface,
  Event,
  EventContextInterface,
  Identity,
  ProductType,
  ProductTypeElement,
  ReducerEventContextInterface,
  SubscriptionBuilderImpl,
  SubscriptionEventContextInterface,
  SumType,
  SumTypeVariant,
  TableCache,
  TimeDuration,
  Timestamp,
  deepEqual,
} from "@clockworklabs/spacetimedb-sdk";
export type Feedback = {
  id: number,
  gameId: number,
  playerId: Identity,
  when: Timestamp,
  message: string,
};

/**
 * A namespace for generated helper functions.
 */
export namespace Feedback {
  /**
  * A function which returns this type represented as an AlgebraicType.
  * This function is derived from the AlgebraicType used to generate this type.
  */
  export function getTypeScriptAlgebraicType(): AlgebraicType {
    return AlgebraicType.createProductType([
      new ProductTypeElement("id", AlgebraicType.createU32Type()),
      new ProductTypeElement("gameId", AlgebraicType.createU32Type()),
      new ProductTypeElement("playerId", AlgebraicType.createIdentityType()),
      new ProductTypeElement("when", AlgebraicType.createTimestampType()),
      new ProductTypeElement("message", AlgebraicType.createStringType()),
    ]);
  }

  export function serialize(writer: BinaryWriter, value: Feedback): void {
    Feedback.getTypeScriptAlgebraicType().serialize(writer, value);
  }

  export function deserialize(reader: BinaryReader): Feedback {
    return Feedback.getTypeScriptAlgebraicType().deserialize(reader);
  }

}


