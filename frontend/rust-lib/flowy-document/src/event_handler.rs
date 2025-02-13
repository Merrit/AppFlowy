use crate::entities::{
  DocumentDataPB, EditParams, EditPayloadPB, ExportDataPB, ExportParams, ExportPayloadPB,
  OpenDocumentPayloadPB,
};
use crate::DocumentManager;
use flowy_error::FlowyError;

use lib_dispatch::prelude::{data_result, AFPluginData, AFPluginState, DataResult};
use std::convert::TryInto;
use std::sync::Arc;

pub(crate) async fn get_document_handler(
  data: AFPluginData<OpenDocumentPayloadPB>,
  manager: AFPluginState<Arc<DocumentManager>>,
) -> DataResult<DocumentDataPB, FlowyError> {
  let context: OpenDocumentPayloadPB = data.into_inner();
  let editor = manager.open_document_editor(&context.document_id).await?;
  let document_data = editor.export().await?;
  data_result(DocumentDataPB {
    doc_id: context.document_id,
    content: document_data,
  })
}

pub(crate) async fn apply_edit_handler(
  data: AFPluginData<EditPayloadPB>,
  manager: AFPluginState<Arc<DocumentManager>>,
) -> Result<(), FlowyError> {
  let params: EditParams = data.into_inner().try_into()?;
  manager.apply_edit(params).await?;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(data, manager), err)]
pub(crate) async fn export_handler(
  data: AFPluginData<ExportPayloadPB>,
  manager: AFPluginState<Arc<DocumentManager>>,
) -> DataResult<ExportDataPB, FlowyError> {
  let params: ExportParams = data.into_inner().try_into()?;
  let editor = manager.open_document_editor(&params.view_id).await?;
  let document_data = editor.export().await?;
  data_result(ExportDataPB {
    data: document_data,
    export_type: params.export_type,
  })
}
